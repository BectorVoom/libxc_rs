//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1446/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1446(t11516: f64, t11547: f64, t1174: f64, t1177: f64, t1178: f64, t1717: f64, t18321: f64, t29614: f64, t3440: f64, t457: f64, t460: f64, t4934: f64, t52281: f64, t6138: f64, t6141: f64, t6147: f64, t73113: f64, t73523: f64, t73535: f64, t73541: f64, t75836: f64, t75912: f64, t78596: f64, t78607: f64, t974: f64) -> f64 {
    let t78634 = -0.50699588477366255142e-1_f64 * t73523 - 0.41152263374485596707e-3_f64 * t52281 + 0.15209876543209876543e0_f64 * t73113 * t1717 - 0.48888888888888888888e-1_f64 * t18321 * t6141 - 0.83333333333333333332e-3_f64 * t1174 * t974 * t457 * (t78596 + t78607) * t460 + 0.13333333333333333332e-1_f64 * t1174 * t3440 * t11547 * t75836 - 0.66666666666666666664e-2_f64 * t1174 * t1177 * t11516 * t75836 - 0.49999999999999999999e-2_f64 * t1174 * t4934 * t29614 * t6138 - 0.27777777777777777777e-3_f64 * t1174 * t1177 * t1178 * t75912 + 0.11111111111111111111e-2_f64 * t73535 - 0.22222222222222222221e-2_f64 * t73541 - 0.48888888888888888888e-1_f64 * t18321 * t6147;
    t78634
}
