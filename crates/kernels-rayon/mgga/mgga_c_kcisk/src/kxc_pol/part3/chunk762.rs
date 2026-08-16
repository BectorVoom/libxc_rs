//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 762/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk762(t11738: f64, t5289: f64, t11671: f64, t7430: f64, t7429: f64, t11658: f64, t740: f64, t1950: f64, t1945: f64, t5332: f64, t10522: f64, t642: f64) -> (f64, f64, f64, f64, f64) {
    let t11739 = t5289 * t11738;
    let t11741 = t7430 * t11671;
    let t11742 = t7429 * t11741;
    let t11744 = t11658 * t740;
    let t11745 = t11744 * t1950;
    let t11747 = t1945 * t5332;
    let t11749 = t642 * t10522;
    (t11739, t11742, t11745, t11747, t11749)
}
