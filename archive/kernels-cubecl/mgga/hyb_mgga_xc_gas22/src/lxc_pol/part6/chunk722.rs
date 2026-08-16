//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 722/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk722<F: Float>(t1128: F, t510: F, t1134: F, t521: F, t2821: F, t2829: F, t2834: F, t2838: F, t2875: F, t2881: F, t2922: F, t2927: F, t3661: F, t3665: F, t3669: F, t3673: F, t3677: F, t3680: F, t3684: F, t3688: F, t3698: F, t3702: F, t3706: F, t3713: F, t3714: F) -> (F, F, F) {
    let t3717 = t510 * t1128;
    let t3720 = t1134 * t521;
    let t3723 = -F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t3661 * t3665 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2821 * t3669 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2829 * t3673 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2834 * t3677 - F::cast_from(50.0_f64) / F::cast_from(3.0_f64) * t3680 * t3665 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2838 * t3684 - F::cast_from(50.0_f64) / F::cast_from(3.0_f64) * t3688 * t3665 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2834 * t3669 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t2838 * t3673 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2821 * t3677 - F::cast_from(18.0_f64) * t2922 * t3698 + F::cast_from(21.0_f64) * t2875 * t3702 - F::cast_from(2.0_f64) * t2927 * t3706 - F::cast_from(2.0_f64) * t2927 * t3698 + F::cast_from(3.0_f64) * t2881 * t3702 - F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t3713 * t3714 - F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t3717 * t3714 - F::cast_from(100.0_f64) / F::cast_from(3.0_f64) * t3720 * t3714;
    (t3717, t3720, t3723)
}
