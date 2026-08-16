//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1085/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1085(t1614: f64, t3282: f64, t1664: f64, t3285: f64, t289: f64, t75689: f64, t75692: f64, t75695: f64, t75700: f64, t75703: f64, t75718: f64, t77774: f64, t77775: f64, t77782: f64, t77785: f64, t77788: f64, t77791: f64, t77792: f64, t77793: f64, t77794: f64, t884: f64) -> (f64, f64) {
    let t80294 = t3282 * t1614;
    let t80297 = t1664 * t3285;
    let t80300 = t77774 + t77775 - 0.81756761766873046873e-6_f64 * t75689 + 0.52557918278704101561e-6_f64 * t75692 + 0.87596530464506835932e-6_f64 * t75695 - 0.87596530464506835932e-6_f64 * t75700 + 0.17519306092901367187e-6_f64 * t75703 - t77782 + 0.59871208509319042821e-1_f64 * t884 * t80294 - t77785 + t77788 + t77791 - t75718 - 0.2363e1_f64 * t289 * t80297 - t77792 + t77793 + t77794;
    (t80294, t80300)
}
