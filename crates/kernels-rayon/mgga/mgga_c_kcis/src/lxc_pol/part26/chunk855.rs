//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 855/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk855(t238: f64, t5992: f64, t86: f64, t2026: f64, t752: f64, t3393: f64, t5973: f64, t5981: f64, t8931: f64, t1628: f64, t6220: f64, t4314: f64, t6188: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17645 = 0.53062222222222222222e-1_f64 * t86 * t238 * t5992;
    let t17676 = t752 * t2026;
    let t17685 = 0.35374814814814814814e-1_f64 * t3393 * t5973;
    let t17686 = t8931 * t5981;
    let t17710 = t6220 * t1628;
    let t17730 = t6188 * t4314;
    (t17645, t17676, t17685, t17686, t17710, t17730)
}
