//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2636/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2636(t22398: f64, t225: f64, t1243: f64, t72361: f64, t1235: f64, t22298: f64, t11907: f64, t11914: f64, t11915: f64, t1215: f64, t15027: f64, t15245: f64, t19128: f64, t19129: f64, t19131: f64, t19142: f64, t19157: f64, t19160: f64, t22341: f64, t22348: f64, t22354: f64, t22372: f64, t22390: f64, t3604: f64, t3624: f64, t44724: f64, t44726: f64, t5064: f64, t53565: f64) -> (f64, f64, f64, f64) {
    let t73613 = t22398 * t225;
    let t73630 = t72361 * t1243;
    let t73663 = t1235 * t22298;
    let t73670 = 24.0_f64 * t1215 * t22348 * t44724 * t44726 + t11914 * t11915 * t73663 - 3.0_f64 * t19128 * t22354 * t3624 - 3.0_f64 * t11907 * t22372 + 12.0_f64 * t15027 * t19142 - 6.0_f64 * t15245 * t19131 - 3.0_f64 * t15245 * t19160 + 3.0_f64 * t19129 * t5064 - 18.0_f64 * t19157 * t53565 + 3.0_f64 * t22341 * t3604 + 3.0_f64 * t22390 * t3604;
    (t73613, t73630, t73663, t73670)
}
