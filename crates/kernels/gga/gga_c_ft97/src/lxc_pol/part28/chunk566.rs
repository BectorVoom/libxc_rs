//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 566/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk566<F: Float>(t25693: F, t384: F, t25692: F, t401: F, t920: F, t423: F, t428: F, t22540: F, t3076: F, t1742: F, t3188: F, t5570: F, t53: F, t22515: F, t1737: F, t3057: F, t5571: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t25694 = t25693 * t384;
    let t25695 = t25692 * t25694;
    let t25698 = t920 * t401;
    let t25699 = t423 * t25698;
    let t25703 = t920 * t428;
    let t25704 = t423 * t25703;
    let t25708 = t3076 * t22540;
    let t25709 = t1742 * t3188;
    let t25710 = t5570 * t25709;
    let t25713 = t920 * t53;
    let t25714 = t423 * t25713;
    let t25715 = t22515 * t25714;
    let t25718 = t1737 * t3188;
    let t25719 = t5570 * t25718;
    let t25722 = t5571 * t3057;
    (t25694, t25695, t25698, t25699, t25703, t25704, t25708, t25709, t25710, t25714, t25715, t25718, t25719, t25722)
}
