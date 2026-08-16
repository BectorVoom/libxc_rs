//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 945/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk945(t2024: f64, t30453: f64, t35204: f64, t35208: f64, t35226: f64, t35230: f64, t35239: f64, t39609: f64, t45670: f64, t45672: f64, t45674: f64, t45676: f64, t45678: f64, t45686: f64, t45688: f64, t45696: f64, t45701: f64, t45709: f64, t504: f64, t739: f64, t9927: f64) -> f64 {
    let t45711 = 0.85129199786595678796e-5_f64 * t45670 - 0.25538759935978703639e-4_f64 * t45672 + 0.25538759935978703639e-4_f64 * t45674 + 0.85129199786595678796e-5_f64 * t45676 - 0.85129199786595678796e-5_f64 * t45678 + 0.11974241701863808564e0_f64 * t739 * t2024 * t30453 + 0.25538759935978703638e-4_f64 * t45686 - 0.24829349937757072983e-4_f64 * t45688 - 0.14408463291498358381e-2_f64 * t39609 - 0.19957069503106347607e-1_f64 * t504 * t9927 - 0.212822999466489197e-4_f64 * t45696 + 0.3192344991997337955e-4_f64 * t45701 - 0.19211284388664477842e-2_f64 * t35204 + 0.46116394948205481339e-3_f64 * t35208 + 0.30487649791575028314e-3_f64 * t35226 - 0.43368970657079495312e-4_f64 * t35230 + t35239 + 0.36021158228745895953e-3_f64 * t45709;
    t45711
}
