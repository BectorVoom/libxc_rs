//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1304/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1304<F: Float>(t1054: F, t2405: F, t3723: F, t24352: F, t2920: F, t35894: F, t10105: F, t3724: F, t10343: F, t11695: F, t36009: F, t36011: F, t36013: F, t36017: F, t36020: F, t36022: F, t36025: F, t36028: F, t36030: F, t36034: F) -> F {
    let t36037 = t1054 * t3723 * t2405;
    let t36040 = t2920 * t24352 * t35894;
    let t36042 = t10105 * t3724;
    let t36044 = t10343 * t11695;
    let t36046 = -F::cast_from(0.16414765573575218917e-4_f64) * t36009 + F::cast_from(0.7113065081882594864e-4_f64) * t36011 + F::cast_from(0.7113065081882594864e-4_f64) * t36013 + F::cast_from(0.14678726495025884871e-5_f64) * t36017 - F::cast_from(0.82073827867876094584e-5_f64) * t36020 + F::cast_from(0.82073827867876094584e-5_f64) * t36022 + F::cast_from(0.23485962392041415794e-5_f64) * t36025 + F::cast_from(0.16414765573575218917e-4_f64) * t36028 - F::cast_from(0.10960115782952660704e-4_f64) * t36030 + F::cast_from(0.16414765573575218917e-4_f64) * t36034 - F::cast_from(0.82073827867876094584e-5_f64) * t36037 - F::cast_from(0.11399142759427235359e-6_f64) * t36040 + F::cast_from(0.18968173551686919637e-3_f64) * t36042 - F::cast_from(0.10829185621455873591e-5_f64) * t36044;
    t36046
}
