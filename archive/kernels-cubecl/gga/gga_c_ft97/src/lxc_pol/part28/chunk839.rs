//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 839/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk839<F: Float>(t7318: F, t938: F, t32152: F, t6441: F, t34434: F, t7195: F, t32233: F, t6449: F, t32242: F, t930: F, t378: F, t1642: F, t925: F) -> (F, F, F, F, F, F, F) {
    let t34451 = t7318 * t938;
    let t34455 = t32152 * t6441;
    let t34458 = t7195 * t34434;
    let t34461 = t32233 * t6449;
    let t34468 = t32242 * t930;
    let t34472 = t378 * t938;
    let t34476 = t1642 * t925;
    (t34451, t34455, t34458, t34461, t34468, t34472, t34476)
}
