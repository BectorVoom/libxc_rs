//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 903/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk903<F: Float>(t457: F, t86052: F, t86102: F, t86140: F, t86168: F, t91: F, t446: F, t447: F, t85474: F, t85456: F, t38262: F, t86090: F, t7793: F, t85825: F, t20182: F, t925: F) -> (F, F, F, F, F, F) {
    let t86172 = t91 * t457 * (t86052 + t86102 + t86140 + t86168);
    let t86175 = t446 * t447 * t85474;
    let t86178 = t446 * t447 * t85456;
    let t86181 = t446 * t38262 * t86090;
    let t86188 = t446 * t7793 * t85825;
    let t86193 = t925 * t20182;
    (t86172, t86175, t86178, t86181, t86188, t86193)
}
