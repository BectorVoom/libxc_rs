//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1401/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1401<F: Float>(t1007: F, t1014: F, t10873: F, t11003: F, t11008: F, t25427: F, t2576: F, t2609: F, t30205: F, t30207: F, t30209: F, t30211: F, t30213: F, t30215: F, t30221: F, t3591: F, t3597: F, t3606: F, t9064: F, t9274: F, t9296: F) -> F {
    let t30366 = t30205 + t30207 + t30209 - t30211 + t30213 + t30215 + F::new(0.23392894490538584828e1) * t1014 * t2576 * t11003 * t1007 - F::new(0.20508037716432813316e4) * t2609 * t10873 - F::new(0.70178683471615754484e1) * t2609 * t11008 - F::new(0.69263436422725855034e2) * t9296 * t3606 + F::new(0.46785788981077169656e1) * t3591 * t9274 + F::new(0.46785788981077169656e1) * t9296 * t3597 - F::new(0.41016075432865626631e4) * t25427 * t9064 * t30221;
    t30366
}
