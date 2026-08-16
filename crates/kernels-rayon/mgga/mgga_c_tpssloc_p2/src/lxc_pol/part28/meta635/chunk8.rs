//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2019/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2019(t91135: f64, t91137: f64, t91140: f64, t91149: f64, t91154: f64, t91158: f64, t91161: f64, t91167: f64, t91170: f64, t91133: f64, t91143: f64, t91145: f64, t91147: f64, t91163: f64, t91165: f64, t91173: f64, t91176: f64, t91179: f64) -> f64 {
    let t93644 = 7.0_f64 / 144.0_f64 * t91135;
    let t93645 = 7.0_f64 / 144.0_f64 * t91137;
    let t93646 = 0.80745512188280781706e-3_f64 * t91140;
    let t93650 = 119.0_f64 / 864.0_f64 * t91149;
    let t93651 = 0.13457585364713463618e-3_f64 * t91154;
    let t93652 = 0.26915170729426927236e-3_f64 * t91158;
    let t93653 = 7.0_f64 / 144.0_f64 * t91161;
    let t93656 = 0.22608743412718618878e-1_f64 * t91167;
    let t93657 = 7.0_f64 / 12.0_f64 * t91170;
    let t93661 = 5.0_f64 / 192.0_f64 * t91133 + t93644 + t93645 - t93646 - 0.80745512188280781706e-3_f64 * t91143 - t91145 / 96.0_f64 - t91147 / 192.0_f64 - t93650 + t93651 - t93652 + t93653 - t91163 / 192.0_f64 - t91165 / 192.0_f64 - t93656 - t93657 + t91173 / 4.0_f64 + t91176 / 8.0_f64 - 0.23739180583354549822e0_f64 * t91179;
    t93661
}
