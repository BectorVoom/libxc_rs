//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 616/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk616(t5061: f64, t740: f64, t5063: f64, t747: f64, t746: f64, t745: f64, t1872: f64, t641: f64, t79: f64, t5068: f64, t4797: f64, t1948: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5315 = t5061 * t740;
    let t5316 = t747 * t5063;
    let t5317 = t746 * t5316;
    let t5318 = t5315 * t5317;
    let t5320 = t740 * t745;
    let t5321 = t1872 * t5320;
    let t5322 = t79 * t641;
    let t5323 = t5322 * t5068;
    let t5324 = t5321 * t5323;
    let t5326 = t747 * t4797;
    let t5327 = t746 * t5326;
    let t5328 = t1948 * t5327;
    (t5317, t5318, t5320, t5321, t5322, t5323, t5324, t5327, t5328)
}
