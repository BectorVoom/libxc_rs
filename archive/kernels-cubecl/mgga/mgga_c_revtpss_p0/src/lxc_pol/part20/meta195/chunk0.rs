//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 956/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk956<F: Float>(t4021: F, t9976: F, t1398: F, t1412: F, t3938: F, t3992: F, t2661: F, t1353: F, t3889: F, t4012: F, t828: F, t1384: F) -> (F, F, F, F, F, F, F) {
    let t9977 = t9976 * t4021;
    let t9979 = t1412 * t1398;
    let t9980 = t9979 * t3938;
    let t9981 = t3992 * t9980;
    let t9982 = t2661 * t9981;
    let t9984 = t3889 * t1353;
    let t9986 = t4012 * t828 * t9984;
    let t9989 = t1384 * t1384;
    (t9977, t9979, t9981, t9982, t9984, t9986, t9989)
}
