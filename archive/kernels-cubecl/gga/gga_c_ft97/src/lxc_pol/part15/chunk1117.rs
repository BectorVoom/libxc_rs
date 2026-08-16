//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1117/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1117<F: Float>(t1096: F, t21373: F, t5009: F, t206: F, t21251: F, t6816: F, t7853: F, t4960: F, t5025: F, t214: F, t52: F, t4977: F) -> (F, F, F, F, F, F, F) {
    let t88433 = t1096 * t21373;
    let t88439 = t5009 * t5009;
    let t88442 = F::cast_from(1.0_f64) / t206 / t21251 / t6816;
    let t88444 = t88439 * t88442 * t7853;
    let t88447 = t4960 * t5025;
    let t88456 = t52 * t214 / t206 / t6816;
    let t88462 = t4960 * t4977;
    (t88433, t88439, t88442, t88444, t88447, t88456, t88462)
}
