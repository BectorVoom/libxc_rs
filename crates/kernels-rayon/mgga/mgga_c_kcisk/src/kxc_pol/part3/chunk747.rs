//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 747/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk747(t5111: f64, t960: f64, t10442: f64, t1835: f64, t5114: f64, t965: f64, t11513: f64, t11516: f64, t11519: f64, t11524: f64, t11528: f64, t11532: f64, t11533: f64, t11535: f64, t158: f64, t165: f64, t173: f64) -> f64 {
    let t11537 = t960 * t5111;
    let t11539 = t1835 * t10442;
    let t11542 = t965 * t5114;
    let t11544 = -0.4755e-2_f64 * t165 * t11513 - 0.30247875e-4_f64 * t173 * t11516 - 0.1585e-2_f64 * t165 * t11519 - t11524 + t11528 + t11532 - 0.32788e-1_f64 * t11533 + 0.10566666666666666666e-1_f64 * t11535 - 0.28104e-1_f64 * t11537 - 0.21078e-1_f64 * t158 * t11539 + 0.79249999999999999999e-2_f64 * t11542;
    t11544
}
