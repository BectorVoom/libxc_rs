//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1002/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1002(t13301: f64, t1769: f64, t9528: f64, t2861: f64, t5020: f64, t5010: f64, t5014: f64, t1747: f64, t3225: f64, t2822: f64, t5006: f64, t5000: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13302 = 0.14739506172839506172e-2_f64 * t13301;
    let t13303 = t9528 * t1769;
    let t13305 = t2861 * t5020;
    let t13307 = t2861 * t5010;
    let t13308 = 0.22109259259259259258e-2_f64 * t13307;
    let t13312 = t2861 * t5014;
    let t13321 = t1747 * t3225;
    let t13322 = t13321 * sigma0;
    let t13382 = t2822 * t5006;
    let t13391 = t2822 * t5000;
    (t13302, t13303, t13305, t13307, t13308, t13312, t13321, t13322, t13382, t13391)
}
