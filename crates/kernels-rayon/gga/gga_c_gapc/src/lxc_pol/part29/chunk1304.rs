//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1304/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1304(t1054: f64, t2405: f64, t3723: f64, t24352: f64, t2920: f64, t35894: f64, t10105: f64, t3724: f64, t10343: f64, t11695: f64, t36009: f64, t36011: f64, t36013: f64, t36017: f64, t36020: f64, t36022: f64, t36025: f64, t36028: f64, t36030: f64, t36034: f64) -> f64 {
    let t36037 = t1054 * t3723 * t2405;
    let t36040 = t2920 * t24352 * t35894;
    let t36042 = t10105 * t3724;
    let t36044 = t10343 * t11695;
    let t36046 = -0.16414765573575218917e-4_f64 * t36009 + 0.7113065081882594864e-4_f64 * t36011 + 0.7113065081882594864e-4_f64 * t36013 + 0.14678726495025884871e-5_f64 * t36017 - 0.82073827867876094584e-5_f64 * t36020 + 0.82073827867876094584e-5_f64 * t36022 + 0.23485962392041415794e-5_f64 * t36025 + 0.16414765573575218917e-4_f64 * t36028 - 0.10960115782952660704e-4_f64 * t36030 + 0.16414765573575218917e-4_f64 * t36034 - 0.82073827867876094584e-5_f64 * t36037 - 0.11399142759427235359e-6_f64 * t36040 + 0.18968173551686919637e-3_f64 * t36042 - 0.10829185621455873591e-5_f64 * t36044;
    t36046
}
