//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 765/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk765(t5277: f64, t718: f64, t5291: f64, t10436: f64, t7303: f64, t7302: f64, t11236: f64, t740: f64, t5317: f64, t1931: f64, t5299: f64, t11225: f64, t732: f64) -> (f64, f64, f64, f64, f64) {
    let t11763 = t5277 * t718;
    let t11764 = t11763 * t5291;
    let t11766 = t7303 * t10436;
    let t11767 = t7302 * t11766;
    let t11769 = t11236 * t740;
    let t11770 = t11769 * t5317;
    let t11772 = t1931 * t5299;
    let t11774 = t732 * t11225;
    (t11764, t11767, t11770, t11772, t11774)
}
