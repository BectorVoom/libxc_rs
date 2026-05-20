//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1170/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1170<F: Float>(t13881: F, t13882: F, t13884: F, t13889: F, t225: F, t1392: F, t73: F, t13768: F, t3829: F, t1412: F, t5591: F, t1353: F) -> (F, F, F, F) {
    let t13892 = (t13881 + t13882 + t13884 + t13889) * t225;
    let t13902 = t1392 * t73;
    let t13907 = t13768 * t3829;
    let t13910 = t1412 * t5591;
    let t13911 = t13910 * t1353;
    (t13892, t13902, t13907, t13911)
}
