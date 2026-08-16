//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1206/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1206(t32342: f64, t10627: f64, t161: f64, t1845: f64, t21488: f64, t320: f64, t795: f64, t10701: f64, t1841: f64, t10632: f64, t5524: f64, t2925: f64, t935: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32343 = 0.32043859292259267849e-3_f64 * t32342;
    let t32348 = t10627 * t161;
    let t32349 = t32348 * t1845;
    let t32351 = 0.11963040802443459997e-1_f64 * t21488 * t320 * t795 * t32349;
    let t32352 = t1841 * t10701;
    let t32353 = 0.85450291446024714264e-3_f64 * t32352;
    let t32355 = 0.25635087433807414278e-2_f64 * t5524 * t10632;
    let t32356 = t2925 * t935;
    (t32343, t32348, t32349, t32351, t32353, t32355, t32356)
}
