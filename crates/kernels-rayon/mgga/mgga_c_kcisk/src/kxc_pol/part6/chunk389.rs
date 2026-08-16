//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 389/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk389(t1801: f64, t2527: f64, t1873: f64, t1869: f64, t2454: f64, t719: f64, t717: f64, t415: f64, t1899: f64, t2441: f64, t1800: f64, t1693: f64, t1796: f64, t2399: f64, t2470: f64, t2475: f64, t2511: f64, t671: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2528 = t1801 * t2527;
    let t2529 = t1873 * t2528;
    let t2530 = t1869 * t2529;
    let t2532 = sigma2 * t2454;
    let t2533 = t2532 * t719;
    let t2534 = t717 * t2533;
    let t2535 = t415 * t2534;
    let t2537 = t1899 * t2441;
    let t2538 = t1800 * t2537;
    let t2539 = t1869 * t2538;
    let t2541 = t2399 * t671 - 0.193e0_f64 * t1693 * t2470 + t1796 + 0.16581944444444444444e-2_f64 * t2475 + 0.24872916666666666666e-2_f64 * t2511 - 0.24872916666666666666e-2_f64 * t2530 - 0.66327777777777777776e-2_f64 * t2535 + 0.16581944444444444444e-2_f64 * t2539;
    (t2528, t2529, t2530, t2532, t2533, t2534, t2535, t2537, t2538, t2539, t2541)
}
