//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 601/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk601<F: Float>(t22585: F, t25688: F, t420: F, t423: F, t373: F, t920: F, t384: F, t401: F, t428: F, t22540: F, t3076: F, t1742: F, t3188: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25689 = t22585 * t25688;
    let t25692 = t420 * t423;
    let t25693 = t920 * t373;
    let t25694 = t25693 * t384;
    let t25695 = t25692 * t25694;
    let t25698 = t920 * t401;
    let t25699 = t423 * t25698;
    let t25703 = t920 * t428;
    let t25704 = t423 * t25703;
    let t25708 = t3076 * t22540;
    let t25709 = t1742 * t3188;
    (t25689, t25694, t25695, t25698, t25699, t25703, t25704, t25708, t25709)
}
