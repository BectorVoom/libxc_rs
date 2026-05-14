//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1070/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1070<F: Float>(t2405: F, t27787: F, t6118: F, t9744: F, t24543: F, t27802: F, t2354: F, t2413: F, t2: F, t27742: F, t684: F, t27816: F, t96925: F, t27750: F, t10157: F, t24546: F, t3837: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t108247 = t6118 * t9744 * t27787 * t2405;
    let t108249 = t24543 * t27802;
    let t108250 = t108249 / 9.0;
    let t108253 = t6118 * t2354 * t27787 * t2413;
    let t108255 = t2 * t27742;
    let t108258 = t6118 * t2354 * t108255 * t684;
    let t108260 = t96925 * t27816;
    let t108261 = t108260 / 3.0;
    let t108262 = t24543 * t27750;
    let t108263 = 2.0 * t108262;
    let t108266 = t6118 * t10157 * t24546 * t3837;
    (t108247, t108249, t108250, t108253, t108258, t108260, t108261, t108262, t108263, t108266)
}
