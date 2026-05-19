//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 727/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk727<F: Float>(t1762: F, t5964: F, t424: F, t625: F, t1764: F, t1768: F, t1693: F, t5714: F, t61: F, t1793: F, t410: F, t1669: F, t1673: F) -> (F, F, F, F, F, F, F) {
    let t5966 = F::cast_from(0.96319466275353142157e0_f64) * t1762 * t5964;
    let t5967 = t424 * t625;
    let t5968 = t5967 * t1764;
    let t5970 = t5967 * t1768;
    let t5972 = t424 * t1693;
    let t5975 = F::cast_from(0.11558335953042377058e2_f64) * t61 * t5714;
    let t5976 = t410 * t1793;
    let t5978 = t1673 * t1669;
    (t5966, t5968, t5970, t5972, t5975, t5976, t5978)
}
