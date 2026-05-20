//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3057/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3057<F: Float>(t2444: F, t4534: F, t689: F, t198: F, t2394: F, t4567: F, t588: F, t15183: F, t698: F, t15172: F, t2439: F, t4625: F) -> (F, F, F, F, F, F) {
    let t51759 = t689 * t2444 * t4534;
    let t51780 = t198 * t2394;
    let t51835 = F::new(12.0) * t4567 * t588;
    let t51909 = t698 * t15183;
    let t51911 = t698 * t15172;
    let t51913 = t2439 * t4625;
    (t51759, t51780, t51835, t51909, t51911, t51913)
}
