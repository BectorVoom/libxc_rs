//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1311/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1311<F: Float>(t10450: F, t680: F, t2018: F, t3990: F, t10472: F, t677: F, t136: F, t1815: F, t3946: F, t10168: F, t1240: F, t1243: F, t139: F, t2002: F, t2024: F, t2027: F, t2028: F, t214: F, t23809: F, t24021: F, t24026: F, t24426: F, t24439: F, t26: F, t28136: F, t28150: F, t28153: F, t28156: F, t28628: F, t2949: F, t2950: F, t3287: F, t675: F, t684: F, t687: F, t8536: F) -> F {
    let t28634 = t10450 * t680;
    let t28636 = t3990 * t2018;
    let t28638 = t677 * t10472;
    let t28642 = t136 * t1815 * t3946;
    let t28645 = -F::new(3.0) / F::new(32.0) * t1240 * t8536 + F::new(3.0) / F::new(8.0) * t2949 * t2950 * t3287 + t24021 / F::new(24.0) - t684 * t687 * t28136 * t675 / F::new(32.0) - t684 * t687 * t10168 * t2002 / F::new(64.0) - t2024 * t2027 * t10168 * t2028 / F::new(48.0) - t28150 / F::new(96.0) - t28153 / F::new(96.0) - t28156 / F::new(96.0) + t2024 * t23809 * t1243 / F::new(12.0) - t24026 / F::new(32.0) - F::new(3.0) / F::new(64.0) * t136 * t26 * t139 * t28628 * t214 - t28634 / F::new(32.0) - t28636 / F::new(32.0) - t28638 / F::new(16.0) - t24426 / F::new(32.0) + t28642 / F::new(48.0) - t24439 / F::new(32.0);
    t28645
}
