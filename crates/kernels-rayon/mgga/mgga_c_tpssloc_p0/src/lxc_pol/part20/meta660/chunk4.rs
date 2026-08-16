//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2468/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2468(t14562: f64, t225: f64, t10160: f64, t10170: f64, t10182: f64, t1052: f64, t1066: f64, t11010: f64, t11084: f64, t11085: f64, t14529: f64, t14545: f64, t14549: f64, t1634: f64, t1635: f64, t3020: f64, t3026: f64, t3174: f64, t3207: f64, t388: f64, t43431: f64, t4657: f64, t4660: f64, t4665: f64, t4694: f64) -> f64 {
    let t50653 = t14562 * t225;
    let t50678 = 2.0_f64 * t1052 * t11084 * t1634 * t3174 + 3.0_f64 * t3020 * t388 * t4657 + 12.0_f64 * t10160 * t4665 - 3.0_f64 * t10170 * t4694 + 6.0_f64 * t10182 * t4660 - 6.0_f64 * t1066 * t50653 + 6.0_f64 * t11010 * t4665 - 3.0_f64 * t11010 * t4694 - t11085 * t4660 - 3.0_f64 * t14529 * t3207 - 3.0_f64 * t14545 * t3207 + 6.0_f64 * t14549 * t3026 - 3.0_f64 * t1635 * t43431;
    t50678
}
