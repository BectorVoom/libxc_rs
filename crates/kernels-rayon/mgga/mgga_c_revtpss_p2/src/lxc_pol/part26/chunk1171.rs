//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1171/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1171(t7385: f64, t9292: f64, t2772: f64, t689: f64, t7384: f64, t2722: f64, t7398: f64, t2435: f64, t26447: f64, t11009: f64, t2061: f64, t2067: f64, t231: f64, t25317: f64, t25383: f64, t25407: f64, t25416: f64, t26473: f64, t26493: f64, t26573: f64, t2723: f64, t2828: f64, t7070: f64, t7076: f64, t7414: f64, t7424: f64, t836: f64, t93118: f64, t93244: f64, t95576: f64, t95594: f64, t95598: f64, t95604: f64) -> (f64, f64) {
    let t95607 = 0.17073386770573548589e-1_f64 * t9292 * t7385;
    let t95613 = t689 * t7384 * t2772;
    let t95615 = t7398 * t2722;
    let t95620 = t2435 * t26447;
    let t95622 = -0.28912093960683998208e-1_f64 * t95576 - 0.4336814094102599731e0_f64 * t93244 * t2067 - 0.13010442282307799193e1_f64 * t25407 * t7424 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t26473 * t836 * t231 + 0.10408353825846239354e2_f64 * t7070 * t93118 * t2061 * t11009 - 0.43368140941025997312e-1_f64 * t95594 - 0.21684070470512998656e-1_f64 * t95598 + 0.52041769129231196772e1_f64 * t25383 * t26493 + 0.13010442282307799193e1_f64 * t25383 * t26573 + 0.77108554593144223218e-1_f64 * t95604 - t95607 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t7414 * t2828 - 0.32927245914677557992e-1_f64 * t95613 - 0.26020884564615598386e1_f64 * t7070 * t25416 * t95615 * t2723 + 0.21951497276451705329e-1_f64 * t95620;
    (t95615, t95622)
}
