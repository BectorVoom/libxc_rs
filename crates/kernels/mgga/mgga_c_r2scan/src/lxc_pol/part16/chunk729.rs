//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 729/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk729<F: Float>(t1793: F, t406: F, t1416: F, t661: F, t2036: F, t410: F, t230: F, t4885: F, t1654: F, t761: F, t2049: F, t597: F) -> (F, F, F, F, F, F) {
    let t5980 = t406 * t1793;
    let t5982 = t1416 * t661;
    let t5985 = F::new(12.0) * t410 * t2036;
    let t5986 = t4885 * t230;
    let t5998 = t1654 * t761;
    let t6001 = t597 * t2049;
    (t5980, t5982, t5985, t5986, t5998, t6001)
}
