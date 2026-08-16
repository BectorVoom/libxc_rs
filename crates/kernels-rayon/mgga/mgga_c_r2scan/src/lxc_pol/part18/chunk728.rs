//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 728/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk728(t1762: f64, t5964: f64, t424: f64, t625: f64, t1764: f64, t1768: f64, t1693: f64, t5714: f64, t61: f64, t1793: f64, t410: f64, t1669: f64, t1673: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5966 = 0.96319466275353142157e0_f64 * t1762 * t5964;
    let t5967 = t424 * t625;
    let t5968 = t5967 * t1764;
    let t5970 = t5967 * t1768;
    let t5972 = t424 * t1693;
    let t5975 = 0.11558335953042377058e2_f64 * t61 * t5714;
    let t5976 = t410 * t1793;
    let t5978 = t1673 * t1669;
    (t5966, t5968, t5970, t5972, t5975, t5976, t5978)
}
