//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 394/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk394<F: Float>(t1916: F, t1918: F, t572: F, t573: F, t38: F, t603: F, t43: F, t49: F, t68: F, t72: F) -> (F, F, F, F) {
    let t1921 = t1916 * t573 + F::new(3.0) * t1918 * t572;
    let t1923 = t603 * t38;
    let t1925 = t43 * t49 - t68;
    let t1926 = t1925 * t72;
    (t1921, t1923, t1925, t1926)
}
