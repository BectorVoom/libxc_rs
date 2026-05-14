//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1066/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1066<F: Float>(t30412: F, t30452: F, t30457: F, t30463: F, t32461: F, t32462: F, t34366: F, t34373: F, t34392: F, t34394: F, t37062: F, t37065: F, t39254: F, t39256: F, t39262: F, t39264: F, t39268: F) -> (F,) {
    let t41554 = 0.62896184579208304135e-2 * t30412 - t34366 + t32461 - t39254 / 48.0 - t39256 / 24.0 + t32462 - t34373 + 0.62896184579208304138e-3 * t30452 - 0.90035438047946447644e-2 * t30457 - 0.94344276868812456207e-3 * t30463 + 0.68598428988911579156e-2 * t39262 + t37062 - t37065 + 0.52295833333333333333e0 * t34392 - 0.85748036236139473944e-3 * t34394 + 0.12862205435420921092e-2 * t39264 + 0.12862205435420921092e-2 * t39268;
    (t41554,)
}
