//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 507/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk507(t5531: f64, t798: f64, t2059: f64, t3277: f64, t2063: f64, t3289: f64, t1149: f64, t2068: f64, t2270: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t5532 = t798 * t5531;
    let t5562 = t3277 * t2059;
    let t5570 = t3289 * t2063;
    let t5581 = t2068 * t1149;
    let t5606 = t2270 * sigma0;
    (t5532, t5562, t5570, t5581, t5606)
}
