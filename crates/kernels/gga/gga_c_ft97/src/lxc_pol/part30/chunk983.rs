//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 983/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk983<F: Float>(t1882: F, t34062: F, t34067: F, t34227: F, t34139: F, t312: F, t33953: F, t34169: F, t34174: F, t34337: F, t5: F, t140582: F, t6749: F) -> (F, F, F, F, F, F, F, F, F) {
    let t144246 = t1882 * t34062;
    let t144248 = t1882 * t34067;
    let t144250 = t1882 * t34227;
    let t144260 = t1882 * t34139;
    let t144262 = t312 * t33953;
    let t144271 = t1882 * t34169;
    let t144273 = t1882 * t34174;
    let t144289 = t5 * t34337;
    let t149674 = t140582 * t6749;
    (t144246, t144248, t144250, t144260, t144262, t144271, t144273, t144289, t149674)
}
