//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1201/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1201(t225: f64, t24064: f64, t81398: f64, t12026: f64, t1386: f64, t2092: f64, t24092: f64, t24139: f64, t26224: f64, t26989: f64, t3752: f64, t3882: f64, t39919: f64, t568: f64, t7191: f64, t81379: f64, t81386: f64, t81393: f64, t81395: f64) -> f64 {
    let t84700 = t24064 * t225;
    let t84705 = 0.27415567780803773942e-2_f64 * t81398;
    let t84708 = -18.0_f64 * t26224 * t26989 * t12026 - 0.49348022005446793095e-1_f64 * t81379 + 3.0_f64 * t3752 * t7191 * t568 + 0.9869604401089358619e-1_f64 * t81386 - 18.0_f64 * t3882 * t24092 - 0.23029076935875170111e0_f64 * t81393 - 3.0_f64 * t84700 * t1386 - t39919 * t2092 + 0.23029076935875170111e0_f64 * t81395 - t84705 - 3.0_f64 * t3882 * t24139;
    t84708
}
