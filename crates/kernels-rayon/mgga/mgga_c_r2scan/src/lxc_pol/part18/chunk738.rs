//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 738/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk738(t122: f64, t6100: f64, t261: f64, t277: f64, t254: f64, t2132: f64, t2195: f64, t1598: f64, t2120: f64, t524: f64, t1569: f64, t481: f64) -> (f64, f64, f64, f64, f64) {
    let t6101 = t122 * t6100;
    let t6103 = t261 * t6101 * t277;
    let t6105 = 0.19776387377308997907e1_f64 * t254 * t6103;
    let t6106 = t2195 * t2132;
    let t6118 = t524 * t1598 * t2120;
    let t6121 = t1569 * t481;
    (t6101, t6105, t6106, t6118, t6121)
}
