//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1426/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1426<F: Float>(t15395: F, t18206: F, t15338: F, t4904: F, t3447: F, t3431: F, t6126: F, t1174: F, t6130: F, t11539: F, t6119: F, t4889: F, t4896: F) -> (F, F, F, F, F, F) {
    let t18443 = t15395 * t18206;
    let t18446 = t15338 * t4904;
    let t18447 = t3447 * t18446;
    let t18451 = t3431 * t6126;
    let t18452 = t1174 * t18451;
    let t18454 = t3431 * t6130;
    let t18455 = t1174 * t18454;
    let t18457 = t11539 * t6119;
    let t18458 = t1174 * t18457;
    let t18460 = t4889 * t4896;
    (t18443, t18447, t18452, t18455, t18458, t18460)
}
