//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1169/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1169<F: Float>(t22230: F, t22234: F, t22290: F, t22294: F, t22297: F, t27358: F, t27361: F, t27370: F, t27373: F, t31240: F, t31242: F, t31250: F, t31254: F, t31258: F, t31262: F, t31265: F, t31268: F, t31271: F, t31273: F) -> (F,) {
    let t31510 = -0.28179666666666666667e1 * t22230 + t22234 - 0.22076e1 * t22290 + t22294 + t22297 + 0.258925e1 * t31240 + 0.16504875e0 * t31242 + 0.82785e0 * t27358 - 0.99342e0 * t27361 - 0.49671e0 * t27370 - 0.49671e0 * t27373 + 0.745065e0 * t31250 + 0.745065e0 * t31254 + 0.248355e0 * t31258 + 0.248355e0 * t31262 - 0.49671e0 * t31265 - 0.16557e0 * t31268 + 0.58258125e1 * t31271 - 0.1237865625e0 * t31273;
    (t31510,)
}
