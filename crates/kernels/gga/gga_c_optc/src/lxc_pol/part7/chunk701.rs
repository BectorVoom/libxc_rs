//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 701/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk701<F: Float>(t2007: F, t2017: F, t5: F, t629: F, t6560: F, t2003: F, t627: F, t631: F, t135: F, t2011: F, t628: F, t636: F, t6901: F, t6904: F, t6907: F, t6910: F, t6913: F, t6919: F, t6925: F, t6928: F, t6933: F, t6938: F, t6942: F, t6945: F, t6947: F) -> (F, F, F) {
    let t6950 = t2007 * t2017;
    let t6953 = t629 * t5 * t6560;
    let t6956 = t2003 * t627;
    let t6957 = t6956 * t631;
    let t6959 = 0.15213032607130326246e0 * t6901 - 0.10866451862235947318e-1 * t135 * t6904 - 0.86207184773738515394e0 * t6907 + 3.0 / 16.0 * t2011 * t6910 - 0.76065163035651631229e0 * t6913 - 0.32599355586707841954e0 * t135 * t6919 - t6925 + 0.32599355586707841954e-1 * t636 * t6928 - 0.16299677793353920977e0 * t636 * t6933 + 0.32599355586707841954e-1 * t636 * t6938 - 7.0 / 16.0 * t6942 - t6945 * t6947 / 4.0 + 7.0 / 48.0 * t6950 - t628 * t6953 / 48.0 - 35.0 / 72.0 * t6957;
    (t6953, t6956, t6959)
}
