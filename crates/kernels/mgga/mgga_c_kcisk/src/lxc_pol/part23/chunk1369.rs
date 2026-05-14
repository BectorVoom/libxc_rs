//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1369/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1369<F: Float>(t19746: F, t5600: F, t9461: F, t1339: F, t32000: F, t5621: F, t5627: F, t113962: F, t32008: F, t109883: F, t18993: F, t3482: F, t1411: F, t14187: F, t33609: F, t394: F) -> (F, F, F, F, F, F) {
    let t114145 = t5600 * t9461 * t19746;
    let t114148 = t1339 * t32000 * t5621;
    let t114151 = t1339 * t32000 * t5627;
    let t114157 = t32008 * t113962;
    let t114162 = t3482 * t109883 * t18993;
    let t114172 = t1411 * t14187 * t394 * t33609;
    (t114145, t114148, t114151, t114157, t114162, t114172)
}
