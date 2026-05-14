//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 591/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk591<F: Float>(t1882: F, t3277: F, t3273: F, t3268: F, t10992: F, t11021: F, t11023: F, t11025: F, t11043: F, t3155: F, t458: F, t1771: F, t963: F, t1775: F, t3135: F, t3128: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t11610 = 2.0 / 27.0 * t1882 * t3277;
    let t11612 = 2.0 / 9.0 * t1882 * t3273;
    let t11632 = 4.0 / 9.0 * t1882 * t3268;
    let t11638 = 2.0 / 27.0 * t10992;
    let t11646 = 2.0 / 27.0 * t11021;
    let t11647 = 4.0 / 27.0 * t11023;
    let t11648 = 4.0 / 81.0 * t11025;
    let t11659 = 4.0 / 81.0 * t11043;
    let t11668 = 2.0 / 3.0 * t458 * t3155;
    let t11669 = t1771 * t963;
    let t11684 = 4.0 / 9.0 * t1775 * t3135;
    let t11686 = 4.0 / 27.0 * t1775 * t3128;
    (t11610, t11612, t11632, t11638, t11646, t11647, t11648, t11659, t11668, t11669, t11684, t11686)
}
