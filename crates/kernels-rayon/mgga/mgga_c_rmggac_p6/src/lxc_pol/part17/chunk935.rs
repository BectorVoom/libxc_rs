//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 935/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk935(t45343: f64, t674: f64, t2007: f64, t321: f64, t9888: f64, t262: f64, t36629: f64, t333: f64, t41634: f64, t39507: f64, t45514: f64, t45519: f64, t45523: f64, t45525: f64, t45527: f64, t45531: f64, t45536: f64, t45541: f64, t45546: f64, t45550: f64, t45554: f64, t45559: f64, t530: f64, t623: f64, t739: f64, t8795: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45561 = t45343 * t674;
    let t45562 = t45561 * t2007;
    let t45568 = t9888 * t321;
    let t45569 = t262 * t45568;
    let t45570 = t36629 * t45569;
    let t45572 = t9888 * t333;
    let t45573 = t262 * t45572;
    let t45574 = t41634 * t45573;
    let t45576 = -0.1064114997332445985e-4_f64 * t45514 + 0.85129199786595678796e-5_f64 * t45519 - 0.53205749866622299248e-5_f64 * t45523 - 0.25538759935978703638e-4_f64 * t45525 - 0.11974241701863808564e0_f64 * t739 * t45527 + 0.34093327067806677161e-2_f64 * t45531 + 0.1064114997332445985e-4_f64 * t45536 - 0.1064114997332445985e-4_f64 * t45541 - 0.85129199786595678796e-5_f64 * t45546 + 0.25538759935978703639e-4_f64 * t45550 - 0.25538759935978703639e-4_f64 * t45554 - 0.25538759935978703639e-4_f64 * t45559 + 0.25538759935978703639e-4_f64 * t45562 - 0.39914139006212695214e-1_f64 * t623 * t8795 - 0.4726e1_f64 * t530 * t39507 + 0.20455996240684006296e0_f64 * t45570 - 0.40911992481368012592e0_f64 * t45574;
    (t45561, t45568, t45569, t45572, t45573, t45576)
}
