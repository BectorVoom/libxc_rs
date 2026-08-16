//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2313/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2313(t40: f64, t12908: f64, t20749: f64, t12923: f64, t4194: f64, t5398: f64, t20800: f64, t262: f64, t10143: f64, t20778: f64, t13115: f64, t16586: f64, t12950: f64, t1430: f64, t16558: f64, t16637: f64, t17635: f64, t20217: f64, t20234: f64, t2291: f64, t3966: f64, t4104: f64, t607: f64, t67060: f64, t75: f64, t767: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t146 = t40 <= zeta_threshold;
    let t67228 = 36.0_f64 * t12908 * t20749;
    let t67230 = t4194 * t12923 * t5398;
    let t67231 = 36.0_f64 * t67230;
    let t67235 = t262 * t20800;
    let t67239 = t20778 * t10143;
    let t67243 = t13115 * t16586;
    let t67244 = 36.0_f64 * t67243;
    let t67262 = piecewise3(t146, 0.0_f64, -56.0_f64 / 81.0_f64 * t2291 * t20234 * t607 + 8.0_f64 / 9.0_f64 * t16637 * t3966 + 8.0_f64 / 9.0_f64 * t1430 * t17635 - 2.0_f64 / 3.0_f64 * t12950 * t5398 - 2.0_f64 / 3.0_f64 * t4104 * t16558 - 2.0_f64 / 9.0_f64 * t75 * t20217 * t607 + 2.0_f64 / 3.0_f64 * t767 * t67060);
    (t67228, t67231, t67235, t67239, t67244, t67262)
}
