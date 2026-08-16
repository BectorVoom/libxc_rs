//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1097/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1097(t1587: f64, t2228: f64, t41922: f64, t4048: f64, t9523: f64, t30204: f64, t36718: f64, t36735: f64, t36748: f64, t36754: f64, t38060: f64, t41891: f64, t41893: f64, t41895: f64, t41897: f64, t41902: f64, t41906: f64, t41915: f64, t41920: f64, t4601: f64, t4965: f64, t739: f64, t9340: f64, t9399: f64) -> (f64, f64, f64) {
    let t43903 = t2228 * t1587;
    let t43911 = 0.11918087970123395032e-3_f64 * t41922;
    let t43914 = t9523 * t4048;
    let t43919 = -0.1702583995731913576e-4_f64 * t41891 - 0.85129199786595678799e-5_f64 * t41893 + 0.212822999466489197e-4_f64 * t41895 + 0.5107751987195740728e-4_f64 * t41897 + 0.85129199786595678799e-5_f64 * t41902 - 0.20455996240684006298e-1_f64 * t41906 - 0.81300399444200075499e-3_f64 * t36718 - 0.11974241701863808564e0_f64 * t739 * t43903 + 0.35922725105591425692e0_f64 * t4601 * t9399 + 0.39726959900411316772e-4_f64 * t36735 + 0.3405167991463827152e-4_f64 * t41915 + 0.638468998399467591e-4_f64 * t41920 + t43911 + 0.79828278012425390428e-1_f64 * t4965 * t9340 + 0.47896966807455234256e0_f64 * t30204 * t43914 - 0.60975299583150056624e-3_f64 * t36748 - t38060 - 0.60975299583150056624e-3_f64 * t36754;
    (t43903, t43914, t43919)
}
