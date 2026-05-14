//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 815/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk815<F: Float>(t1358: F, t13725: F, t2439: F, t5622: F, t9765: F, t5610: F, t9775: F, t1889: F, t9779: F, t1882: F, t4003: F, t1873: F, t9741: F, t5651: F, t808: F, t9736: F) -> (F, F, F, F, F, F, F) {
    let t13726 = t13725 * t1358;
    let t13727 = t2439 * t13726;
    let t13765 = t9765 * t5622;
    let t13779 = t9775 * t5610;
    let t13781 = t9779 * t1889;
    let t13790 = t1882 * t4003;
    let t13798 = t9741 * t1873;
    let t13800 = t808 * t5651;
    let t13801 = t9736 * t13800;
    (t13727, t13765, t13779, t13781, t13790, t13798, t13801)
}
