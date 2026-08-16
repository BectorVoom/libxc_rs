//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1136/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1136(t8333: f64, t973: f64, t2294: f64, t2300: f64, t8344: f64, t970: f64, t346: f64, t349: f64, t8343: f64, t2302: f64, t2315: f64, t23543: f64, t23545: f64, t23551: f64, t23553: f64, t23555: f64, t23557: f64, t23561: f64, t23565: f64, t23567: f64, t23569: f64, t23576: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23691 = t8333 * t973;
    let t23694 = t2294 * t2300;
    let t23699 = t970 * t8344;
    let t23708 = t346 / t8343 / t349;
    let t23709 = t2302 * t2302;
    let t23715 = t2315 * t2315;
    let t23732 = -0.17481481481481481482e3_f64 * t23543 - 0.41955555555555555556e3_f64 * t23545 + 0.41955555555555555555e3_f64 * t23551 + 0.93234567901234567903e3_f64 * t23553 + 0.10488888888888888889e4_f64 * t23555 + 0.12586666666666666667e4_f64 * t23557 - 0.94399999999999999998e3_f64 * t23561 - 0.78666666666666666666e2_f64 * t23565 + 0.20977777777777777778e3_f64 * t23567 + 0.932345679012345679e2_f64 * t23569 - 0.81580246913580246914e2_f64 * t23576;
    (t23691, t23694, t23699, t23708, t23709, t23715, t23732)
}
