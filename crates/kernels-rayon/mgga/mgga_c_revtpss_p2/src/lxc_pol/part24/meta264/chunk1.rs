//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1036/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1036(t372: f64, t5277: f64, t3362: f64, t471: f64, t1285: f64, t12865: f64, t5302: f64, t15904: f64, t3623: f64) -> (f64, f64, f64, f64, f64) {
    let t17661 = t372 * t5277;
    let t17687 = t471 * t3362;
    let t17693 = t1285 * t12865;
    let t17694 = t372 * t5302;
    let t17708 = t3623 * t15904;
    (t17661, t17687, t17693, t17694, t17708)
}
