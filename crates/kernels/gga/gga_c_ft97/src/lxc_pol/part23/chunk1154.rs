//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1154/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1154<F: Float>(t10491: F, t6217: F, t43917: F, t25409: F, t6963: F, t25462: F, t28987: F, t29006: F, t317: F, t9577: F, t28935: F, t28951: F, t28835: F, t683: F, t43912: F, t28557: F, t28676: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t111733 = t10491 * t6217;
    let t111737 = t43917 * t6217;
    let t111743 = 2.0 / 9.0 * t6963 * t25409;
    let t111747 = t25462 * t28987 / 27.0;
    let t111751 = 2.0 / 3.0 * t25462 * t29006;
    let t111783 = t317 * t9577;
    let t111795 = 2.0 / 27.0 * t25462 * t28935;
    let t111801 = 2.0 / 27.0 * t25462 * t28951;
    let t111807 = t683 * t28835;
    let t111815 = t43912 * t6217;
    let t111830 = t28676 * t28557;
    (t111733, t111737, t111743, t111747, t111751, t111783, t111795, t111801, t111807, t111815, t111830)
}
