//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 808/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk808<F: Float>(t16746: F, t572: F, t11: F, t571: F, t16715: F, t16720: F, t16722: F, t16724: F, t16726: F, t16728: F, t16730: F, t16732: F, t16734: F, t16736: F, t16743: F, t16710: F, t173: F, t184: F, t199: F) -> (F, F, F) {
    let t16747 = t572 * t16746;
    let t16749 = t11 * t571 * t16747;
    let t16751 = -0.2518888888888888889e-1 * t16715 + 0.12594444444444444445e-1 * t16720 - 0.10075555555555555556e-1 * t16722 + 0.10075555555555555556e-1 * t16724 - 0.5037777777777777778e-2 * t16726 - 0.78365432098765432099e-2 * t16728 + 0.50377777777777777778e-2 * t16730 + 0.33585185185185185186e-2 * t16732 - 0.27987654320987654323e-2 * t16734 - 0.25188888888888888889e-2 * t16736 + 0.55975308641975308645e-2 * t16743 + 0.18891666666666666667e-2 * t16749;
    let t16756 = 2.0 / 15.0 * t173 * (t16710 + t16751) * t184 * t199;
    (t16747, t16749, t16756)
}
