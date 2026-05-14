//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1143/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1143<F: Float>(t32176: F, t9426: F, t1440: F, t382: F, t1286: F, t32045: F, t1411: F, t3748: F, t9463: F, t1500: F, t394: F) -> (F, F, F, F, F, F) {
    let t32192 = t9426 * t32176;
    let t32196 = t382 * t1440;
    let t32197 = t32196 * t1286;
    let t32198 = t32045 * t32197;
    let t32199 = t1411 * t32198;
    let t32201 = t3748 * t9463;
    let t32203 = t1500 * t394;
    (t32192, t32197, t32198, t32199, t32201, t32203)
}
