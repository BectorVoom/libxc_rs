//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 601/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk601(t22585: f64, t25688: f64, t420: f64, t423: f64, t373: f64, t920: f64, t384: f64, t401: f64, t428: f64, t22540: f64, t3076: f64, t1742: f64, t3188: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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
