//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1222/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1222<F: Float>(t10683: F, t28506: F, t446: F, t824: F, t28501: F, t684: F, t10248: F, t113066: F, t2739: F, t7036: F, t2665: F, t992: F, t99342: F, t28719: F, t668: F, t505: F) -> (F, F, F, F, F, F, F) {
    let t113124 = t446 * t10683 * t28506 * t824;
    let t113126 = t28501 * t684;
    let t113128 = t446 * t10248 * t113126;
    let t113131 = t446 * t10248 * t113066;
    let t113135 = t446 * t10683 * t7036 * t2739;
    let t113139 = t446 * t2665 * t99342 * t992;
    let t113141 = t28719 * t668;
    let t113144 = t446 * t2665 * t113141 * t505;
    (t113124, t113126, t113128, t113131, t113135, t113139, t113144)
}
