//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1190/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1190<F: Float>(t15935: F, t19661: F, t1042: F, t19666: F, t4801: F, t1592: F, t16138: F, t19399: F, t247: F, t3116: F, t18942: F, t4915: F, t1011: F, t1063: F, t11656: F, t11994: F, t11999: F, t16057: F, t16062: F, t16064: F, t3127: F, t4837: F, t6263: F, t6312: F) -> (F,) {
    let t19929 = t15935 * t19661;
    let t19930 = t1042 * t19929;
    let t19933 = t4801 * t19666;
    let t19934 = t1042 * t19933;
    let t19939 = t16138 * t1592;
    let t19940 = t1042 * t19939;
    let t19944 = t247 * t3116 * t19399;
    let t19947 = t4915 * t18942;
    let t19950 = 0.15244095330869239812e-2 * t11656 * t6263 + 0.11433071498151929859e-2 * t11999 * t6312 + 0.85748036236139473944e-3 * t1063 * t19930 - 0.57165357490759649296e-3 * t1063 * t19934 - 0.28582678745379824648e-3 * t11994 * t6263 - 0.28582678745379824648e-3 * t3127 * t19940 + t16057 + t16062 - t16064 + 0.85748036236139473944e-3 * t4837 * t19944 - t1011 * t19947 / 144.0;
    (t19950,)
}
