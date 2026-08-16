//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 780/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk780(t2505: f64, t4597: f64, t2543: f64, t574: f64, t2551: f64, t979: f64, t10879: f64, t2637: f64, t2013: f64, t2630: f64, t5477: f64, t2634: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18036 = t2505 * t4597;
    let t18089 = t2543 * t574;
    let t18132 = t979 * t2551;
    let t18355 = t10879 * t2637;
    let t18356 = t2013 * t18355;
    let t18406 = t2630 * t5477;
    let t18408 = t2634 * t5477;
    (t18036, t18089, t18132, t18356, t18406, t18408)
}
