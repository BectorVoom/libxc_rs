//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 890/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk890<F: Float>(t12505: F, t12526: F, t12547: F, t12568: F, t11043: F, t11046: F, t1125: F, t12464: F, t12466: F, t12476: F, t12479: F, t12483: F, t2464: F, t2469: F, t338: F, t3565: F, t3568: F, t3622: F, t3883: F, t3897: F, t7056: F, t7063: F, t884: F, t972: F) -> (F, F) {
    let t12570 = t12505 + t12526 + t12547 + t12568;
    let t12572 = -2.0 * t11043 * t1125 + 4.0 * t11046 * t3568 + t12464 * t338 - t12466 * t972 - 6.0 * t12476 * t7063 + 4.0 * t12479 * t2469 + 2.0 * t12483 * t2469 - t12570 * t884 - t2464 * t3897 - 2.0 * t3565 * t3622 + 2.0 * t3883 * t7056;
    (t12570, t12572)
}
