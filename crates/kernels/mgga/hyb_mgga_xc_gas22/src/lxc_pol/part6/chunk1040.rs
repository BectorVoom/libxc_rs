//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1040/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1040<F: Float>(t3676: F, t9696: F, t1171: F, t3785: F, t537: F, t1539: F, t2849: F, t9691: F, t3683: F, t2821: F, t2829: F, t2834: F, t2838: F, t3661: F, t3665: F, t3677: F, t3680: F, t3733: F, t9490: F, t9646: F, t9678: F, t9682: F, t9686: F, t9692: F) -> (F, F, F, F, F) {
    let t9697 = t3676 * t9696;
    let t9700 = t3785 * t1171;
    let t9703 = t3785 * t537;
    let t9714 = t2849 * t1539;
    let t9715 = t9714 * t9691;
    let t9718 = t3683 * t9696;
    let t9723 = -F::new(128.0) / F::new(81.0) * t9678 * t9646 - F::new(8.0) / F::new(9.0) * t2821 * t9682 + F::new(8.0) / F::new(9.0) * t2829 * t9686 - F::new(64.0) / F::new(27.0) * t3680 * t9692 - F::new(32.0) / F::new(9.0) * t2834 * t9697 - F::new(16.0) / F::new(9.0) * t9700 * t3677 - F::new(100.0) / F::new(9.0) * t9703 * t3665 - F::new(8.0) / F::new(3.0) * t2834 * t9682 + F::new(8.0) / F::new(3.0) * t2838 * t9686 - F::new(64.0) / F::new(81.0) * t3733 * t9692 - F::new(32.0) / F::new(27.0) * t2821 * t9697 + F::new(64.0) / F::new(81.0) * t3661 * t9715 + F::new(32.0) / F::new(27.0) * t2829 * t9718 + F::new(400.0) / F::new(9.0) * t3680 * t9490;
    (t9700, t9703, t9715, t9718, t9723)
}
