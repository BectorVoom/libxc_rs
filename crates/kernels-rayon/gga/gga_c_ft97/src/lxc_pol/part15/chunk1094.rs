//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1094/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1094(t4888: f64, t4893: f64, t1064: f64, t1078: f64, t16612: f64, t184: f64, t185: f64, t20044: f64, t20990: f64, t21: f64, t21086: f64, t21092: f64, t3601: f64, t3664: f64, t4431: f64, t4845: f64, t4889: f64, t4894: f64, t4898: f64, t5: f64, t623: f64, t85501: f64, t87219: f64, t87814: f64, t920: f64) -> f64 {
    let t87827 = t4888 * t4888;
    let t87835 = t4893 * t4893;
    let t87840 = 3.0_f64 * t3601 * t21092 + t623 * t21086 * t1078 * t3664 + 3.0_f64 / 2.0_f64 * t623 * t4894 * t21 * t4888 + 3.0_f64 / 2.0_f64 * t5 * t4845 * t4431 + t5 * t20990 * t920 + t5 * t185 * t85501 / 4.0_f64 + t5 * (t87219 + t87814) * t184 * t21 / 4.0_f64 + t5 * t1064 * t20044 + 3.0_f64 * t16612 * t4898 + 3.0_f64 / 2.0_f64 * t623 * t4889 * t4431 + 3.0_f64 / 4.0_f64 * t623 * t87827 * t184 * t21 + 3.0_f64 / 2.0_f64 * t623 * t4894 * t4431 + t623 * t87835 * t184 * t21 / 4.0_f64;
    t87840
}
