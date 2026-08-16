//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 721/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk721<F: Float>(t11371: F, t7983: F, t408: F, t929: F, t3020: F, t428: F, t388: F, t939: F, t398: F, t401: F, t51: F, t6: F) -> (F, F, F, F) {
    let t11372 = t7983 * t11371;
    let t11375 = t408 * t929;
    let t11377 = t3020 * t11375 * t428;
    let t11380 = t388 * t939;
    let t11383 = t401 * t6 * t51 * t398;
    (t11372, t11377, t11380, t11383)
}
