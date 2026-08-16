//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1040/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1040(t3676: f64, t9696: f64, t1171: f64, t3785: f64, t537: f64, t1539: f64, t2849: f64, t9691: f64, t3683: f64, t2821: f64, t2829: f64, t2834: f64, t2838: f64, t3661: f64, t3665: f64, t3677: f64, t3680: f64, t3733: f64, t9490: f64, t9646: f64, t9678: f64, t9682: f64, t9686: f64, t9692: f64) -> (f64, f64, f64, f64, f64) {
    let t9697 = t3676 * t9696;
    let t9700 = t3785 * t1171;
    let t9703 = t3785 * t537;
    let t9714 = t2849 * t1539;
    let t9715 = t9714 * t9691;
    let t9718 = t3683 * t9696;
    let t9723 = -128.0_f64 / 81.0_f64 * t9678 * t9646 - 8.0_f64 / 9.0_f64 * t2821 * t9682 + 8.0_f64 / 9.0_f64 * t2829 * t9686 - 64.0_f64 / 27.0_f64 * t3680 * t9692 - 32.0_f64 / 9.0_f64 * t2834 * t9697 - 16.0_f64 / 9.0_f64 * t9700 * t3677 - 100.0_f64 / 9.0_f64 * t9703 * t3665 - 8.0_f64 / 3.0_f64 * t2834 * t9682 + 8.0_f64 / 3.0_f64 * t2838 * t9686 - 64.0_f64 / 81.0_f64 * t3733 * t9692 - 32.0_f64 / 27.0_f64 * t2821 * t9697 + 64.0_f64 / 81.0_f64 * t3661 * t9715 + 32.0_f64 / 27.0_f64 * t2829 * t9718 + 400.0_f64 / 9.0_f64 * t3680 * t9490;
    (t9700, t9703, t9715, t9718, t9723)
}
