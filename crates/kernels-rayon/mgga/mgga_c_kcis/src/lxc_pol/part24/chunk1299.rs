//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1299/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1299(t28932: f64, t7699: f64, t27856: f64, t27895: f64, t15573: f64, t2173: f64, t28996: f64, t100619: f64, t100622: f64, t101057: f64, t19674: f64, t2175: f64, t27808: f64, t27964: f64, t3489: f64, t7703: f64, t8034: f64, t8042: f64, t95524: f64, t96391: f64) -> f64 {
    let t101393 = t28932 * t7699;
    let t101395 = t27895 * t27856;
    let t101402 = t2173 * t15573 * t28996;
    let t101406 = -0.24872916666666666666e-2_f64 * t100619 - 0.33163888888888888888e-2_f64 * t100622 - 0.13901041666666666667e-2_f64 * t7703 * t101057 - 0.55652820312500000001e-3_f64 * t95524 * t27808 + 0.18534722222222222222e-2_f64 * t19674 * t3489 * t2175 - 0.23168402777777777778e-3_f64 * t101393 + 0.6183646701388888889e-4_f64 * t101395 - 0.37069444444444444445e-2_f64 * t27964 * t8042 - 0.49469173611111111112e-3_f64 * t96391 * t8034 - 0.46336805555555555557e-3_f64 * t101402 - 0.37069444444444444445e-2_f64 * t27964 * t8034;
    t101406
}
