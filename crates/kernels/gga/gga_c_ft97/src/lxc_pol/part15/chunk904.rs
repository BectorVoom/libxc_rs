//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 904/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk904<F: Float>(t446: F, t7824: F, t86193: F, t57435: F, t73256: F, t73259: F, t73262: F, t73276: F, t73299: F, t73301: F, t86016: F, t86020: F, t86172: F, t86175: F, t86178: F, t86181: F, t86188: F) -> (F, F) {
    let t86195 = t446 * t7824 * t86193;
    let t86197 = -15.0 / 16.0 * t86016 - 3.0 / 4.0 * t86020 + t86172 / 2.0 - 12.0 * t86175 + 8.0 / 3.0 * t86178 + 8.0 * t86181 - 4.0 / 3.0 * t73256 + 8.0 / 3.0 * t73259 - 16.0 / 9.0 * t73262 + 4.0 / 9.0 * t73276 + 8.0 / 3.0 * t86188 + 16.0 / 9.0 * t57435 + 8.0 / 3.0 * t73299 + 8.0 / 3.0 * t73301 - 8.0 * t86195;
    (t86195, t86197)
}
