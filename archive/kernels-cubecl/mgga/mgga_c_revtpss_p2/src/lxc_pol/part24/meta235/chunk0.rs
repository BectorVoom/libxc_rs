//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 994/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk994<F: Float>(t1408: F, t240: F, t5610: F, t9775: F, t1889: F, t9779: F, t828: F, t9954: F, t3935: F, t1882: F, t4003: F) -> (F, F, F, F, F, F) {
    let t13767 = t1408 * t240;
    let t13779 = t9775 * t5610;
    let t13781 = t9779 * t1889;
    let t13783 = t9954 * t828;
    let t13789 = t3935 * t828;
    let t13790 = t1882 * t4003;
    (t13767, t13779, t13781, t13783, t13789, t13790)
}
