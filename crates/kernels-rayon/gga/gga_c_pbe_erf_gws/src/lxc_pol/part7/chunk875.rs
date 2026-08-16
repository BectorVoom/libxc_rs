//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 875/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk875(t16746: f64, t572: f64, t11: f64, t571: f64, t16715: f64, t16720: f64, t16722: f64, t16724: f64, t16726: f64, t16728: f64, t16730: f64, t16732: f64, t16734: f64, t16736: f64, t16743: f64) -> (f64, f64, f64) {
    let t16747 = t572 * t16746;
    let t16749 = t11 * t571 * t16747;
    let t16751 = -0.2518888888888888889e-1_f64 * t16715 + 0.12594444444444444445e-1_f64 * t16720 - 0.10075555555555555556e-1_f64 * t16722 + 0.10075555555555555556e-1_f64 * t16724 - 0.5037777777777777778e-2_f64 * t16726 - 0.78365432098765432099e-2_f64 * t16728 + 0.50377777777777777778e-2_f64 * t16730 + 0.33585185185185185186e-2_f64 * t16732 - 0.27987654320987654323e-2_f64 * t16734 - 0.25188888888888888889e-2_f64 * t16736 + 0.55975308641975308645e-2_f64 * t16743 + 0.18891666666666666667e-2_f64 * t16749;
    (t16747, t16749, t16751)
}
