//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 698/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk698(t1282: f64, t187: f64, t1872: f64, t2205: f64, t3669: f64, t437: f64, t5360: f64, t7809: f64, t8061: f64, t8062: f64, t8063: f64, t8066: f64, t8082: f64, t8104: f64, t8108: f64, t8117: f64) -> f64 {
    let t8121 = t8061 - t8062 - t8063 + t8066 - t8082 + t187 * (-t1282 * t8117 - t1872 * t7809 - t2205 * t5360 + 2.0_f64 * t3669 * t8108 + t437 * t8104 - t8061 + t8062 + t8063 - t8066 + t8082);
    t8121
}
