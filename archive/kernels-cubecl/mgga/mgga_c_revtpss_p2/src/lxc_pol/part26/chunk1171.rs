//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1171/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1171<F: Float>(t7385: F, t9292: F, t2772: F, t689: F, t7384: F, t2722: F, t7398: F, t2435: F, t26447: F, t11009: F, t2061: F, t2067: F, t231: F, t25317: F, t25383: F, t25407: F, t25416: F, t26473: F, t26493: F, t26573: F, t2723: F, t2828: F, t7070: F, t7076: F, t7414: F, t7424: F, t836: F, t93118: F, t93244: F, t95576: F, t95594: F, t95598: F, t95604: F) -> (F, F) {
    let t95607 = F::cast_from(0.17073386770573548589e-1_f64) * t9292 * t7385;
    let t95613 = t689 * t7384 * t2772;
    let t95615 = t7398 * t2722;
    let t95620 = t2435 * t26447;
    let t95622 = -F::cast_from(0.28912093960683998208e-1_f64) * t95576 - F::cast_from(0.4336814094102599731e0_f64) * t93244 * t2067 - F::cast_from(0.13010442282307799193e1_f64) * t25407 * t7424 + F::cast_from(0.13010442282307799193e1_f64) * t7070 * t7076 * t26473 * t836 * t231 + F::cast_from(0.10408353825846239354e2_f64) * t7070 * t93118 * t2061 * t11009 - F::cast_from(0.43368140941025997312e-1_f64) * t95594 - F::cast_from(0.21684070470512998656e-1_f64) * t95598 + F::cast_from(0.52041769129231196772e1_f64) * t25383 * t26493 + F::cast_from(0.13010442282307799193e1_f64) * t25383 * t26573 + F::cast_from(0.77108554593144223218e-1_f64) * t95604 - t95607 - F::cast_from(0.78062653693846795158e1_f64) * t7070 * t25317 * t7414 * t2828 - F::cast_from(0.32927245914677557992e-1_f64) * t95613 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25416 * t95615 * t2723 + F::cast_from(0.21951497276451705329e-1_f64) * t95620;
    (t95615, t95622)
}
