//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2001/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2001(t1579: f64, t26550: f64, t103005: f64, t25375: f64, t2718: f64, t7398: f64, t26506: f64, t27216: f64, t14587: f64, t25317: f64, t25391: f64, t25394: f64, t27349: f64, t27353: f64, t2828: f64, t28426: f64, t28439: f64, t28442: f64, t7070: f64, t8006: f64, t92917: f64, t93349: f64, t95720: f64, t95722: f64, t95727: f64, t95732: f64, t95733: f64, t95825: f64, t99237: f64) -> f64 {
    let t103037 = t26550 * t1579;
    let t103047 = 0.28912093960683998208e-1_f64 * t25375 * t103005;
    let t103059 = t2718 * t7398;
    let t103063 = t27216 * t26506;
    let t103065 = -0.14456046980341999104e-1_f64 * t95720 + 0.52041769129231196772e1_f64 * t93349 * t103037 * t25394 + 0.8673628188205199462e0_f64 * t99237 * t28439 - 0.17347256376410398924e1_f64 * t25391 * t95825 * t27349 - t103047 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t8006 * t2828 + 0.38549458614245330943e-1_f64 * t95722 - 0.68540937416128198418e-2_f64 * t95727 - t95732 + 0.25702851531048074406e-1_f64 * t95733 - 0.17347256376410398924e1_f64 * t92917 * t28442 - 0.17347256376410398924e1_f64 * t99237 * t28426 - 0.17347256376410398924e1_f64 * t27353 * t103059 * t14587 + 0.17135234354032049604e-1_f64 * t103063;
    t103065
}
