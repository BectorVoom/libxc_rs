//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1250/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1250<F: Float>(t15935: F, t19661: F, t1042: F, t19666: F, t4801: F, t1592: F, t16138: F, t19399: F, t247: F, t3116: F, t18942: F, t4915: F) -> (F, F, F, F, F) {
    let t19929 = t15935 * t19661;
    let t19930 = t1042 * t19929;
    let t19933 = t4801 * t19666;
    let t19934 = t1042 * t19933;
    let t19939 = t16138 * t1592;
    let t19940 = t1042 * t19939;
    let t19944 = t247 * t3116 * t19399;
    let t19947 = t4915 * t18942;
    (t19930, t19934, t19940, t19944, t19947)
}
