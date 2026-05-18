//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 735/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk735<F: Float>(t1117: F, t1123: F, t1129: F, t1134: F, t1145: F, t1536: F, t1540: F, t1543: F, t1546: F, t1549: F, t2829: F, t2868: F, t2903: F, t2922: F, t3665: F, t3684: F, t3706: F, t3714: F, t3724: F, t3727: F, t3733: F, t3739: F, t3743: F, t3747: F, t3749: F, t3753: F, t3757: F, t3760: F, t3767: F, t3772: F, t3779: F, t3786: F, t3788: F, t510: F, t518: F) -> F {
    let t3791 = -F::new(100.0) / F::new(3.0) * t3724 * t3714 + F::new(15.0) * t2868 * t1145 * t3727 - F::new(18.0) * t2922 * t3706 - F::new(50.0) / F::new(9.0) * t3733 * t3665 + F::new(8.0) / F::new(9.0) * t2829 * t3684 - F::new(32.0) / F::new(81.0) * t3739 * t3743 - F::new(16.0) / F::new(27.0) * t3747 * t3749 - F::new(32.0) / F::new(81.0) * t3753 * t3743 - F::new(16.0) / F::new(27.0) * t3757 * t3749 - F::new(36.0) * t1134 * t3760 * t1129 - F::new(36.0) * t1134 * t1546 * t1123 + F::new(42.0) * t518 * t3767 * t1129 - F::new(4.0) * t1117 * t3772 - F::new(4.0) * t1117 * t1540 * t1123 + F::new(6.0) * t510 * t3779 + F::new(30.0) * t2903 * t1543 * t1123 + t3786 * t1549 + F::new(2.0) * t3788 * t1536;
    t3791
}
