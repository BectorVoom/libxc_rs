//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 683/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk683(t10568: f64, t311: f64, t3841: f64, t579: f64, t10585: f64, t4726: f64, t26: f64, t10593: f64, t1659: f64, t10570: f64, t10572: f64, t10574: f64, t10576: f64, t10579: f64, t10582: f64, t10587: f64, t10590: f64, t10595: f64, t10598: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10639 = 0.93932222222222222223e0_f64 * t10568;
    let t10641 = t311 * t3841 * t579;
    let t10642 = 0.36793333333333333333e0_f64 * t10641;
    let t10643 = t4726 * t10585;
    let t10644 = t26 * t10643;
    let t10646 = t1659 * t10593;
    let t10647 = t26 * t10646;
    let t10649 = 28.0_f64 / 27.0_f64 * t10568;
    let t10660 = -t10649 - 4.0_f64 / 9.0_f64 * t10570 + 2.0_f64 / 9.0_f64 * t10572 - 2.0_f64 / 3.0_f64 * t10574 + t10576 / 3.0_f64 - 10.0_f64 / 27.0_f64 * t10579 + 4.0_f64 / 3.0_f64 * t10582 - 2.0_f64 / 3.0_f64 * t10587 - 2.0_f64 * t10590 + 2.0_f64 * t10595 - t10598 / 3.0_f64;
    (t10639, t10641, t10642, t10644, t10647, t10660)
}
