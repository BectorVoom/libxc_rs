//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 881/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk881<F: Float>(t12297: F, t12299: F, t12301: F, t12303: F, t12307: F, t12310: F, t12314: F, t12317: F, t12320: F, t12344: F, t12347: F, t12354: F, t12542: F, t12543: F, t12531: F, t1188: F) -> (F, F) {
    let t12546 = 0.20128333333333333333e0 * t12299 + 0.33547222222222222222e0 * t12307 + 0.40256666666666666668e0 * t12297 - 0.60385000000000000001e0 * t12301 - 0.30192500000000000001e0 * t12303 - 0.12077e1 * t12310 + 0.181155e1 * t12314 + 0.301925e0 * t12320 - 0.3883875e1 * t12344 + 0.247573125e0 * t12347 - t12542 - t12543 + 0.181155e1 * t12317 + 0.16504875e0 * t12354;
    let t12547 = t12531 + t12546;
    let t12548 = t12547 * t1188;
    (t12547, t12548)
}
