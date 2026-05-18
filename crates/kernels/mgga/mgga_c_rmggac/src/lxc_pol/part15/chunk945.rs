//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 945/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk945<F: Float>(t2024: F, t30453: F, t35204: F, t35208: F, t35226: F, t35230: F, t35239: F, t39609: F, t45670: F, t45672: F, t45674: F, t45676: F, t45678: F, t45686: F, t45688: F, t45696: F, t45701: F, t45709: F, t504: F, t739: F, t9927: F) -> F {
    let t45711 = F::new(0.85129199786595678796e-5) * t45670 - F::new(0.25538759935978703639e-4) * t45672 + F::new(0.25538759935978703639e-4) * t45674 + F::new(0.85129199786595678796e-5) * t45676 - F::new(0.85129199786595678796e-5) * t45678 + F::new(0.11974241701863808564e0) * t739 * t2024 * t30453 + F::new(0.25538759935978703638e-4) * t45686 - F::new(0.24829349937757072983e-4) * t45688 - F::new(0.14408463291498358381e-2) * t39609 - F::new(0.19957069503106347607e-1) * t504 * t9927 - F::new(0.212822999466489197e-4) * t45696 + F::new(0.3192344991997337955e-4) * t45701 - F::new(0.19211284388664477842e-2) * t35204 + F::new(0.46116394948205481339e-3) * t35208 + F::new(0.30487649791575028314e-3) * t35226 - F::new(0.43368970657079495312e-4) * t35230 + t35239 + F::new(0.36021158228745895953e-3) * t45709;
    t45711
}
