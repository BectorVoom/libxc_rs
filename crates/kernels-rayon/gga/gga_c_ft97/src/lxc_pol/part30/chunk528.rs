//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 528/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk528(t18968: f64, t3700: f64, t18: f64, t2639: f64, t342: f64, t5202: f64, t630: f64, t231: f64, t4129: f64, t10207: f64, t10209: f64, t10212: f64, t13616: f64, t1526: f64, t15567: f64, t18959: f64, t18962: f64, t2320: f64, t343: f64, t4027: f64, t4037: f64, t4052: f64, t4135: f64) -> f64 {
    let t18969 = t18968 * t3700;
    let t18972 = t2639 * t18;
    let t18977 = t342 * t630 * t5202;
    let t18982 = t231 * t4129;
    let t18986 = t4027 + t4135 + t10207 - t10209 / 36.0_f64 - t10212 / 12.0_f64 - t18959 / 36.0_f64 - t15567 * t18962 / 9.0_f64 - t1526 * t2320 * t4037 / 12.0_f64 + t15567 * t18969 / 6.0_f64 + t1526 * t13616 * t18972 / 6.0_f64 - t18977 / 12.0_f64 - t1526 * t2320 * t4052 / 12.0_f64 - t342 * t343 * t18982 / 4.0_f64;
    t18986
}
