//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 998/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk998(t50: f64, t17463: f64, t17548: f64, t17659: f64, t18223: f64, t16241: f64, t1438: f64, t1440: f64, t1591: f64, t1593: f64, t17322: f64, t17329: f64, t208: f64, t367: f64, t368: f64, t501: f64, t502: f64, t5080: f64, t5084: f64, t5479: f64, t5483: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t51 = t50 <= zeta_threshold;
    let t18225 = t17463 + t17548 + t17659 + t18223;
    let t18232 = piecewise3(t51, 0.0_f64, t16241);
    let t18236 = t208 * (t17322 * t368 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t5080 * t1440 + 3.0_f64 / 2.0_f64 * t1438 * t5084 + t367 * t17329 / 2.0_f64 + t18225 * t502 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t5479 * t1593 + 3.0_f64 / 2.0_f64 * t1591 * t5483 + t501 * t18232 / 2.0_f64);
    (t18225, t18232, t18236)
}
