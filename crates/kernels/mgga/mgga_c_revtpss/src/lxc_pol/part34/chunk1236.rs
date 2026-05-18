//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1236/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1236<F: Float>(t15822: F, t25508: F, t25516: F, t4954: F, t25504: F, t4857: F, t7131: F, t3201: F, t7801: F, t15670: F, t1972: F, t15749: F, t7117: F) -> (F, F, F, F, F, F, F) {
    let t100063 = t15822 * t25508;
    let t100146 = t4954 * t25516;
    let t100173 = t15822 * t25504;
    let t100255 = t4857 * t7131;
    let t100272 = t7801 * t3201;
    let t100321 = t15670 * t1972;
    let t100329 = t7117 * t15749;
    (t100063, t100146, t100173, t100255, t100272, t100321, t100329)
}
