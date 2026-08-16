//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 395/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk395(t2586: f64, t747: f64, t741: f64, t2441: f64, t641: f64, t746: f64, t2561: f64, t2565: f64, t2569: f64, t2573: f64, t2577: f64, t2581: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2587 = t2586 * t747;
    let t2588 = t741 * t2587;
    let t2590 = t641 * t2441;
    let t2591 = t746 * t2590;
    let t2592 = t741 * t2591;
    let t2594 = t2561 / 16.0_f64 - t2565 / 16.0_f64 - t2569 / 6.0_f64 + t2573 / 24.0_f64 - t2577 / 256.0_f64 + t2581 / 256.0_f64 + t2588 / 48.0_f64 - t2592 / 192.0_f64;
    (t2587, t2588, t2590, t2591, t2592, t2594)
}
