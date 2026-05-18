//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1184/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1184<F: Float>(t231: F, t25317: F, t25383: F, t26515: F, t26551: F, t2771: F, t7070: F, t7076: F, t7398: F, t92917: F, t95615: F, t95779: F, t95783: F, t95786: F, t95790: F, t95794: F, t95796: F, t95798: F, t95807: F, t95808: F, t95811: F, t95813: F) -> F {
    let t95821 = F::new(0.39029762157531132076e-1) * t95779 - F::new(0.72280234901709995519e-3) * t95783 - F::new(0.51405703062096148812e-1) * t95786 + F::new(0.38554277296572111609e-1) * t95790 + F::new(0.51405703062096148814e-2) * t95794 + F::new(0.28912093960683998208e-1) * t95796 - F::new(0.21684070470512998656e-1) * t95798 + F::new(0.13010442282307799193e1) * t7070 * t7076 * t95615 * t231 + F::new(0.13010442282307799193e1) * t25383 * t26515 + t95807 - F::new(0.68549505033305214441e-2) * t95808 + F::new(0.72280234901709995519e-3) * t95811 - F::new(0.68549505033305214441e-2) * t95813 - F::new(0.52041769129231196772e1) * t92917 * t26551 - F::new(0.78062653693846795158e1) * t7070 * t25317 * t7398 * t2771;
    t95821
}
