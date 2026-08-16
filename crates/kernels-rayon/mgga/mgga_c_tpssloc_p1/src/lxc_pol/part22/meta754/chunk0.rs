//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2533/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2533(t449: f64, t71289: f64, t71308: f64, t1671: f64, t63750: f64, t18686: f64, t4782: f64, t14845: f64, t6021: f64, t18835: f64, t4740: f64, t11310: f64, t11350: f64, t1136: f64, t1155: f64, t15171: f64, t15225: f64, t18612: f64, t18616: f64, t18640: f64, t18786: f64, t4835: f64, t51382: f64, t51389: f64, t51727: f64, t6052: f64, t6084: f64, t71095: f64, t71097: f64, t71217: f64, t71245: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71310 = (t71289 + t71308) * t449;
    let t71313 = 3.0_f64 * t63750 * t1671;
    let t71315 = 3.0_f64 * t18686 * t4782;
    let t71317 = 3.0_f64 * t14845 * t6021;
    let t71319 = 3.0_f64 * t4740 * t18835;
    let t71322 = -t71095 + 0.10526802520742363173e2_f64 * t51389 * t18612 + t71097 + 0.62071215503128080361e4_f64 * t11350 * t6052 * t15171 * t1136 + 18.0_f64 * t51382 * t18640 + 0.30762056574649219973e4_f64 * t11310 * t6084 * t15225 * t1155 + t71217 - 0.31168546390226634766e3_f64 * t51727 * t18616 - 0.19751673498613801407e-1_f64 * t71310 - t71245 - t71313 - t71315 - t71317 - t71319 + 0.17544670867903938621e1_f64 * t4835 * t18786;
    (t71310, t71313, t71315, t71317, t71319, t71322)
}
