//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 237/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk237<F: Float>(t143: F, t172: F, t187: F, t694: F, t708: F, t712: F, t716: F, t720: F, t724: F, t728: F, t732: F, t736: F, t740: F, t759: F) -> F {
    let t144 = F::new(0.135e1) <= t143;
    let t763 = piecewise3::<F>(t144, -t694 * t708 / F::new(18.0) + t712 * t708 / F::new(240.0) - t716 * t708 / F::new(4480.0) + t720 * t708 / F::new(103680.0) - t724 * t708 / F::new(2838528.0) + t728 * t708 / F::cast_from(89456640.0_f64) - t732 * t708 / F::new(0.31850496e10) + t736 * t708 / F::cast_from(0.1263403008e12_f64), -F::new(8.0) / F::new(3.0) * t172 * t759 - F::new(8.0) / F::new(3.0) * t740 * t187);
    t763
}
