//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2645/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2645(t53777: f64, t53779: f64, t56099: f64, t56102: f64, t56104: f64, t20396: f64, t67: f64, t758: f64, t53798: f64, t5397: f64, t606: f64, t584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73958 = 0.65061487801810439052e-1_f64 * t53777;
    let t73959 = 0.97592231702715658578e-1_f64 * t53779;
    let t73960 = 0.51947577317044391276e2_f64 * t56099;
    let t73961 = 0.17544670867903938621e1_f64 * t56102;
    let t73962 = 0.17544670867903938621e1_f64 * t56104;
    let t73967 = t20396 * t67 * t758;
    let t73968 = 0.18311447306006545054e-3_f64 * t73967;
    let t73969 = 0.10526802520742363173e2_f64 * t53798;
    let t73975 = t5397 * t606;
    let t73978 = t584 * t5397;
    (t73958, t73959, t73960, t73961, t73962, t73968, t73969, t73975, t73978)
}
