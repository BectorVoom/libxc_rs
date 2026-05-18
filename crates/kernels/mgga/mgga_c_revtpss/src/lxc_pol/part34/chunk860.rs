//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 860/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk860<F: Float>(t15014: F, t2439: F, t1569: F, t2453: F, t2458: F, t2435: F, t4322: F, t1596: F, t2873: F, t1614: F, t2942: F, t1606: F) -> (F, F, F, F, F, F) {
    let t15015 = t2439 * t15014;
    let t15017 = t2453 * t1569;
    let t15018 = t15017 * t2458;
    let t15063 = t2435 * t4322;
    let t15101 = t1596 * t2873;
    let t15104 = t1614 * t2942;
    let t15123 = t2439 * t1606;
    (t15015, t15018, t15063, t15101, t15104, t15123)
}
