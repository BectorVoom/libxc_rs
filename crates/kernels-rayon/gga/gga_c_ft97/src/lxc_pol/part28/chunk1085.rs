//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1085/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1085(t1882: f64, t34634: f64, t34624: f64, t34746: f64, t34742: f64, t34762: f64, t103654: f64, t110: f64, t11810: f64, t1307: f64, t137891: f64, t137900: f64, t137906: f64, t145658: f64, t145705: f64, t1871: f64, t1901: f64, t23339: f64, t26061: f64, t26113: f64, t26134: f64, t26154: f64, t26198: f64, t26445: f64, t3103: f64, t3113: f64, t32082: f64, t32333: f64, t32545: f64, t3271: f64, t39120: f64, t446: f64, t452: f64, t47659: f64, t488: f64, t5644: f64, t7281: f64, t83: f64, t91739: f64, t986: f64) -> f64 {
    let t146498 = t1882 * t34634;
    let t146505 = t1882 * t34624;
    let t146520 = t1882 * t34746;
    let t146522 = t1882 * t34742;
    let t146527 = t1882 * t34762;
    let t146547 = 4.0_f64 / 3.0_f64 * t446 * t1871 * t986 * t32082 + t137891 / 9.0_f64 + 2.0_f64 / 9.0_f64 * t137900 + t446 * t452 * t32545 * t3271 / 3.0_f64 + 2.0_f64 / 9.0_f64 * t146498 + 2.0_f64 / 3.0_f64 * t446 * t452 * t488 * t1307 * t26113 - 2.0_f64 / 9.0_f64 * t146505 - 4.0_f64 / 9.0_f64 * t137906 + 2.0_f64 / 9.0_f64 * t1901 * t39120 * t32333 * t3113 - 4.0_f64 / 3.0_f64 * t1901 * t11810 * t23339 * t26154 + 2.0_f64 / 3.0_f64 * t446 * t1871 * t110 * t145658 - 4.0_f64 / 9.0_f64 * t146520 + 2.0_f64 / 9.0_f64 * t146522 + 4.0_f64 / 9.0_f64 * t47659 * t91739 * t26198 + t146527 / 9.0_f64 + t446 * t452 * t488 * t7281 * t3103 / 3.0_f64 - t446 * t83 * t145705 / 3.0_f64 + 4.0_f64 / 9.0_f64 * t47659 * t91739 * t26445 + 4.0_f64 / 9.0_f64 * t47659 * t103654 * t26134 + 2.0_f64 / 3.0_f64 * t446 * t452 * t26061 * t5644;
    t146547
}
