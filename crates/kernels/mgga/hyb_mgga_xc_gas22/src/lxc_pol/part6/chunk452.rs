//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 452/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk452<F: Float>(t151: F, t154: F, t157: F, t160: F, t163: F, t166: F, t169: F, t2042: F, t2070: F, t2098: F, t694: F, t712: F, t716: F, t720: F, t724: F, t728: F, t732: F, t736: F) -> F {
    let t2103 = t151 * t2042 / F::new(6.0) - t694 * t2070 / F::new(18.0) - t154 * t2042 / F::new(48.0) + t712 * t2070 / F::new(240.0) + t157 * t2042 / F::new(640.0) - t716 * t2070 / F::new(4480.0) - t160 * t2042 / F::new(11520.0) + t720 * t2070 / F::new(103680.0) + t163 * t2042 / F::new(258048.0) - t724 * t2070 / F::new(2838528.0) - t166 * t2042 / F::new(6881280.0) + t728 * t2070 / F::cast_from(89456640.0_f64) + t169 * t2042 / F::new(0.21233664e9) - t732 * t2070 / F::new(0.31850496e10) - t2098 * t2042 / F::new(0.74317824e10) + t736 * t2070 / F::cast_from(0.1263403008e12_f64);
    t2103
}
