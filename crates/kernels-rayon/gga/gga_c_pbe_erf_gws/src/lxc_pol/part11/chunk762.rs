//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 762/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk762(t12468: f64, t1758: f64, t11: f64, t12339: f64, t1663: f64, t571: f64, t2554: f64, t3346: f64, t12345: f64, t572: f64, t10823: f64, t10825: f64, t10827: f64, t12462: f64, t12466: f64, t4940: f64, t7374: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12469 = t1758 * t12468;
    let t12470 = t11 * t12469;
    let t12472 = t1663 * t12339;
    let t12473 = t571 * t12472;
    let t12474 = t11 * t12473;
    let t12476 = t2554 * t3346;
    let t12477 = t571 * t12476;
    let t12478 = t11 * t12477;
    let t12480 = t572 * t12345;
    let t12481 = t571 * t12480;
    let t12482 = t11 * t12481;
    let t12484 = t4940 + 0.25188888888888888889e-2_f64 * t7374 - 0.12594444444444444445e-2_f64 * t10823 + 0.37783333333333333335e-2_f64 * t10825 - 0.18891666666666666667e-2_f64 * t10827 + 0.20990740740740740742e-2_f64 * t12462 - 0.75566666666666666669e-2_f64 * t12466 + 0.37783333333333333335e-2_f64 * t12470 + 0.11335e-1_f64 * t12474 - 0.11335e-1_f64 * t12478 + 0.18891666666666666667e-2_f64 * t12482;
    (t12469, t12470, t12472, t12473, t12474, t12476, t12477, t12478, t12480, t12481, t12482, t12484)
}
