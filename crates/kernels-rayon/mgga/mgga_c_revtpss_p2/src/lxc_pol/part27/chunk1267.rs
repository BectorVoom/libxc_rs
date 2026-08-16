//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1267/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1267(t94669: f64, t94671: f64, t25894: f64, t94668: f64, t25950: f64, t25953: f64, t26069: f64, t94407: f64, t1426: f64, t9990: f64, t1444: f64, t2022: f64, t2030: f64, t25921: f64, t26034: f64, t26046: f64, t26079: f64, t4003: f64, t7295: f64, t7296: f64, t94413: f64, t94641: f64, t94643: f64, t94648: f64, t94650: f64, t94656: f64, t94662: f64, t94665: f64, t9658: f64, t9994: f64) -> f64 {
    let t94672 = t94669 * t94671;
    let t94674 = t25894 * t94668;
    let t94675 = t94674 * t94671;
    let t94677 = t25950 * t25953;
    let t94682 = 0.91399340044406952588e-2_f64 * t26069 * t94407;
    let t94683 = t1426 * t9990;
    let t94692 = 0.38554277296572111609e-1_f64 * t94641 - 0.4336814094102599731e0_f64 * t94643 * t2030 + t94648 - 0.15421710918628844643e0_f64 * t94650 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t26034 * t1444 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t2022 * t9658 + 0.57824187921367996415e-1_f64 * t94662 - 0.43368140941025997312e-1_f64 * t94665 - 0.23132566377943266966e0_f64 * t94672 + 0.13010442282307799194e0_f64 * t94675 + 0.51405703062096148812e-1_f64 * t94677 + 0.13010442282307799193e1_f64 * t25921 * t26046 + t94682 + 0.26020884564615598386e1_f64 * t7295 * t94683 * t94413 * t9994 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t94413 * t4003;
    t94692
}
