//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 927/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk927<F: Float>(t230: F, t4977: F, t2440: F, t4939: F, t39976: F, t5249: F, t703: F, t1196: F, t2725: F, t800: F, t2035: F, t5009: F) -> (F, F, F, F, F, F, F) {
    let t70290 = t230 * t4977;
    let t70326 = t2440 * t4939;
    let t70354 = F::new(0.59031789687271907073e-3) * t39976 * t5249;
    let t70402 = t703 * t4977;
    let t70462 = t2725 * t1196;
    let t70463 = t800 * t70462;
    let t70474 = t2035 * t5009;
    (t70290, t70326, t70354, t70402, t70462, t70463, t70474)
}
