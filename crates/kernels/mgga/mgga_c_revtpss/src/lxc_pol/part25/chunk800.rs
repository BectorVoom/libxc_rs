//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 800/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk800<F: Float>(t221: F, t3924: F, t4019: F, t4018: F, t3930: F, t4059: F, t1386: F, t2482: F, t596: F, t4021: F, t1398: F, t1412: F, t3938: F, t3992: F, t2661: F, t1353: F, t3889: F) -> (F, F, F, F, F, F, F) {
    let t9970 = t4019 * t221 * t3924;
    let t9971 = t4018 * t9970;
    let t9973 = t3930 * t4059;
    let t9976 = t2482 * t1386 * t596;
    let t9977 = t9976 * t4021;
    let t9979 = t1412 * t1398;
    let t9980 = t9979 * t3938;
    let t9981 = t3992 * t9980;
    let t9982 = t2661 * t9981;
    let t9984 = t3889 * t1353;
    (t9970, t9971, t9973, t9977, t9980, t9982, t9984)
}
