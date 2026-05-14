//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 963/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk963<F: Float>(t1587: F, t2228: F, t41922: F, t4048: F, t9523: F, t30204: F, t36718: F, t36735: F, t36748: F, t36754: F, t38060: F, t41891: F, t41893: F, t41895: F, t41897: F, t41902: F, t41906: F, t41915: F, t41920: F, t4601: F, t4965: F, t739: F, t9340: F, t9399: F) -> (F, F, F) {
    let t43903 = t2228 * t1587;
    let t43911 = 0.11918087970123395032e-3 * t41922;
    let t43914 = t9523 * t4048;
    let t43919 = -0.1702583995731913576e-4 * t41891 - 0.85129199786595678799e-5 * t41893 + 0.212822999466489197e-4 * t41895 + 0.5107751987195740728e-4 * t41897 + 0.85129199786595678799e-5 * t41902 - 0.20455996240684006298e-1 * t41906 - 0.81300399444200075499e-3 * t36718 - 0.11974241701863808564e0 * t739 * t43903 + 0.35922725105591425692e0 * t4601 * t9399 + 0.39726959900411316772e-4 * t36735 + 0.3405167991463827152e-4 * t41915 + 0.638468998399467591e-4 * t41920 + t43911 + 0.79828278012425390428e-1 * t4965 * t9340 + 0.47896966807455234256e0 * t30204 * t43914 - 0.60975299583150056624e-3 * t36748 - t38060 - 0.60975299583150056624e-3 * t36754;
    (t43903, t43914, t43919)
}
