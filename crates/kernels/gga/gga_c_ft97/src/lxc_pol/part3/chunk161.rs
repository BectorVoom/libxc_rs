//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 161/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk161<F: Float>(t24: F, t432: F, t469: F, t460: F, t462: F, t466: F, t92: F, t457: F, t91: F, t353: F, t366: F, t435: F) -> (F, F, F, F, F) {
    let t471 = t24 * t469 * t432;
    let t473 = -t460 - t462 * t466 / F::new(3.0) - t92 * t471;
    let t475 = t91 * t457 * t473;
    let t477 = t353 / F::new(9.0);
    let t480 = t475 / F::new(6.0) - t477 - t366 / F::new(9.0) - t435 / F::new(3.0);
    (t471, t473, t475, t477, t480)
}
