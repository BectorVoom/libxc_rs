//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 756/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk756<F: Float>(t169: F, t3997: F, t4014: F, t732: F, t2098: F, t736: F, t151: F, t4017: F, t4019: F, t4021: F, t4023: F, t4025: F, t4027: F, t4029: F, t4031: F, t4033: F, t4035: F, t694: F) -> (F, F, F, F, F) {
    let t4037 = t169 * t3997;
    let t4039 = t732 * t4014;
    let t4041 = t2098 * t3997;
    let t4043 = t736 * t4014;
    let t4045 = t151 * t3997 / F::new(6.0) - t694 * t4014 / F::new(18.0) - t4017 / F::new(48.0) + t4019 / F::new(240.0) + t4021 / F::new(640.0) - t4023 / F::new(4480.0) - t4025 / F::new(11520.0) + t4027 / F::new(103680.0) + t4029 / F::new(258048.0) - t4031 / F::new(2838528.0) - t4033 / F::new(6881280.0) + t4035 / F::new(89456640.0) + t4037 / F::new(0.21233664e9) - t4039 / F::new(0.31850496e10) - t4041 / F::new(0.74317824e10) + t4043 / F::new(0.1263403008e12);
    (t4037, t4039, t4041, t4043, t4045)
}
