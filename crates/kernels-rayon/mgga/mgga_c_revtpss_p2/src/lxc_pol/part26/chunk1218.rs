//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1218/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1218(t7496: f64, t9692: f64, t7284: f64, t96370: f64, t26234: f64, t94886: f64, t1398: f64, t1445: f64, t2097: f64, t213: f64, t225: f64, t25921: f64, t26079: f64, t26246: f64, t26333: f64, t26343: f64, t4003: f64, t543: f64, t561: f64, t7295: f64, t7301: f64, t96362: f64, t96405: f64, t96510: f64, t96512: f64, t96516: f64, t96527: f64, t96542: f64, t96546: f64, t9890: f64) -> f64 {
    let t96549 = 0.30356481678079769392e-1_f64 * t7496 * t9692;
    let t96550 = t7284 * t96370;
    let t96552 = t94886 * t26234;
    let t96554 = -0.51405703062096148814e-2_f64 * t96510 - 0.19756347548806534796e1_f64 * t96512 * t1445 - 0.34697458558045176417e-2_f64 * t96516 + 0.65854491829355115987e0_f64 * t213 * t96362 * t225 * t561 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2097 * t9890 * t543 + 0.43368140941025997312e-1_f64 * t96527 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t96405 * t4003 - 0.26020884564615598386e1_f64 * t25921 * t26343 + 0.13010442282307799193e1_f64 * t7295 * t7301 * t26333 * t1398 * t543 + 0.13010442282307799193e1_f64 * t25921 * t26246 - 0.43368140941025997312e-1_f64 * t96542 + 0.14456046980341999104e-2_f64 * t96546 + t96549 + 0.21684070470512998656e-1_f64 * t96550 + 0.15421710918628844643e0_f64 * t96552;
    t96554
}
