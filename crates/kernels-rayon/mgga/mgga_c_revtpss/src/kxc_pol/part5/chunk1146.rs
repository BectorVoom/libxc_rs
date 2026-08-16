//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1146/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1146(t1579: f64, t4533: f64, t2770: f64, t212: f64, t6041: f64, t780: f64, t689: f64, t10498: f64, t10501: f64, t14474: f64, t14479: f64, t14484: f64, t14486: f64, t14985: f64, t14989: f64, t14992: f64, t14995: f64, t865: f64) -> f64 {
    let t18312 = t1579 * t4533;
    let t18313 = t2770 * t18312;
    let t18316 = t212 * t6041;
    let t18317 = t18316 * t780;
    let t18318 = t689 * t18317;
    let t18322 = -0.13009920719177044025e-2_f64 * t14474 - t14479 - t14484 + 0.26019841438354088051e-1_f64 * t14486 + 0.26341796731742046394e1_f64 * t865 * t18313 - 0.54878743191129263322e-2_f64 * t18318 - t14985 - t14989 + 0.39029762157531132076e-1_f64 * t14992 - t14995 + 0.73171657588172351096e-2_f64 * t10498 + t10501;
    t18322
}
