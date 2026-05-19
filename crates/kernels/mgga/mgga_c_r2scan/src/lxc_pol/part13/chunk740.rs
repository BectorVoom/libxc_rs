//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 740/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk740<F: Float>(t1764: F, t5967: F, t1768: F, t1693: F, t424: F, t5714: F, t61: F, t1793: F, t410: F, t1669: F, t1673: F, t1416: F, t661: F) -> (F, F, F, F, F, F, F) {
    let t5968 = t5967 * t1764;
    let t5970 = t5967 * t1768;
    let t5972 = t424 * t1693;
    let t5975 = F::cast_from(0.11558335953042377058e2_f64) * t61 * t5714;
    let t5976 = t410 * t1793;
    let t5978 = t1673 * t1669;
    let t5982 = t1416 * t661;
    (t5968, t5970, t5972, t5975, t5976, t5978, t5982)
}
