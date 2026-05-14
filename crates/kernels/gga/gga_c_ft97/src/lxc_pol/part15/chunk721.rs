//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 721/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk721<F: Float>(t21196: F, t3910: F, t21404: F, t9896: F, t21408: F, t2493: F, t13335: F, t13680: F, t18283: F, t18286: F, t18303: F, t18305: F, t18314: F, t18316: F, t462: F, t9935: F) -> (F, F, F, F) {
    let t21607 = t3910 * t21196;
    let t21610 = t9896 * t21404;
    let t21613 = t2493 * t21408;
    let t21623 = -2.0 / 3.0 * t462 * t21607 - 2.0 * t462 * t21610 - 2.0 * t462 * t21613 + t18286 / 3.0 + t18314 - t9935 - 4.0 / 9.0 * t13335 + 2.0 / 9.0 * t18303 - 2.0 / 3.0 * t18305 - 4.0 / 3.0 * t13680 - 2.0 * t18316 - 2.0 / 3.0 * t18283;
    (t21607, t21610, t21613, t21623)
}
