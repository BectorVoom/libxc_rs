//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 170/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk170<F: Float>(t397: F, t658: F, t662: F, t656: F, t579: F) -> (F, F, F, F, F) {
    let t664 = t397 * t658 * t662;
    let t667 = 1.0 + 0.5397236614853195164e-1 * t656 * t664;
    let t668 = f64::ln(t667);
    let t670 = 1.0 + 0.193e0 * t668;
    let t671 = 1.0 / t670;
    let t673 = 1.0 / t579;
    (t664, t667, t670, t671, t673)
}
