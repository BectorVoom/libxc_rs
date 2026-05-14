//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 906/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk906<F: Float>(t144792: F, t446: F, t7824: F, t144857: F, t38268: F, t34389: F, t376: F, t5665: F, t136308: F, t6449: F, t136303: F, t22513: F, t1554: F, t938: F, t136367: F, t32146: F, t6441: F) -> (F, F, F, F, F, F, F, F) {
    let t145055 = t446 * t7824 * t144792;
    let t145058 = t446 * t38268 * t144857;
    let t145061 = t5665 * t376 * t34389;
    let t145071 = t136308 * t6449;
    let t145074 = t136303 * t6449;
    let t145075 = t22513 * t145074;
    let t145077 = t1554 * t938;
    let t145099 = t32146 * t136367 * t6441;
    (t145055, t145058, t145061, t145071, t145074, t145075, t145077, t145099)
}
