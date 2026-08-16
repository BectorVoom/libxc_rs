//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3830/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3830<F: Float>(t2439: F, t3895: F, t6919: F, t10175: F, t22399: F, t13734: F, t1904: F, t689: F, t2453: F, t3908: F, t6889: F, t22398: F, t2470: F, t3915: F) -> (F, F, F, F, F) {
    let t73641 = t2439 * t3895 * t6919;
    let t73647 = t10175 * t22399;
    let t73652 = t689 * t13734 * t1904;
    let t73656 = t2453 * t6889 * t3908;
    let t73662 = t3915 * t22398 * t2470;
    (t73641, t73647, t73652, t73656, t73662)
}
