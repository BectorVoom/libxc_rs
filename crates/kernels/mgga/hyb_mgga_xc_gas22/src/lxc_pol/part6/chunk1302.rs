//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1302/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1302<F: Float>(t143: F, t28223: F, t28242: F, t28274: F, t28309: F, t10331: F, t151: F, t154: F, t157: F, t160: F, t163: F, t166: F, t169: F, t2098: F, t28162: F, t694: F, t708: F, t712: F, t716: F, t720: F, t724: F, t728: F, t732: F, t736: F) -> (F, F) {
    let t145 = F::new(0.135e1) < t143;
    let t28311 = t28223 + t28242 + t28274 + t28309;
    let t28312 = piecewise3::<f64>(t145, t28311, F::new(0.0));
    let t28335 = -t166 * t28162 / F::new(3440640.0) + t169 * t28162 / F::new(0.10616832e9) - t2098 * t28162 / F::new(0.37158912e10) + t151 * t28162 / F::new(3.0) - t154 * t28162 / F::new(24.0) + t157 * t28162 / F::new(320.0) - t160 * t28162 / F::new(5760.0) + t163 * t28162 / F::new(129024.0) - t694 * t28312 / F::new(18.0) + t712 * t28312 / F::new(240.0) - t716 * t28312 / F::new(4480.0) + t720 * t28312 / F::new(103680.0) - t724 * t28312 / F::new(2838528.0) + t728 * t28312 / F::new(89456640.0) - t732 * t28312 / F::new(0.31850496e10) + t736 * t28312 / F::new(0.1263403008e12) - t154 * t10331 * t708 / F::new(24.0) + t157 * t10331 * t708 / F::new(320.0);
    (t28311, t28335)
}
