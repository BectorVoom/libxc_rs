//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 589/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk589(t147: f64, t184: f64, t4888: f64, t21: f64, t1078: f64, t1079: f64, t920: f64, t1064: f64, t1080: f64, t185: f64, t3601: f64, t4431: f64, t4845: f64, t5: f64, t623: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t148 = 10000000.0_f64 <= t147;
    let t4889 = t4888 * t184;
    let t4890 = t4889 * t21;
    let t4893 = t1078 * t1078;
    let t4894 = t4893 * t184;
    let t4895 = t4894 * t21;
    let t4898 = t1079 * t920;
    let t4905 = piecewise3(t148, 0.0_f64, t5 * t4845 * t21 / 4.0_f64 + t3601 * t1080 / 2.0_f64 + t5 * t1064 * t920 / 2.0_f64 + t623 * t4890 / 4.0_f64 + t623 * t4895 / 4.0_f64 + t623 * t4898 / 2.0_f64 + t5 * t185 * t4431 / 4.0_f64);
    (t4889, t4890, t4893, t4894, t4895, t4898, t4905)
}
