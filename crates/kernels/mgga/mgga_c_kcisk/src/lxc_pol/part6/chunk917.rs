//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 917/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk917<F: Float>(t2455: F, t8793: F, t17277: F, t17327: F, t23326: F, t23779: F, t23802: F, t23805: F, t23808: F, t23811: F, t23814: F, t23840: F, t23843: F, t23858: F, t2466: F, t664: F, t7208: F, t8816: F) -> F {
    let t29310 = t8793 * t2455;
    let t29320 = -F::new(0.28785261945883707541e0) * t23779 + F::new(0.17990788716177317213e-1) * t23802 + F::new(0.2398771828823642295e-1) * t23805 - F::new(0.35981577432354634425e-1) * t23808 + F::new(0.35981577432354634426e-1) * t17277 - F::new(0.10794473229706390328e0) * t23811 + F::new(0.52772980234120130492e0) * t23814 - F::new(0.11993859144118211475e-1) * t17327 - F::new(0.43177892918825561313e0) * t29310 * t664 + F::new(0.28785261945883707541e0) * t23840 + F::new(0.10794473229706390328e0) * t23843 - F::new(0.53972366148531951639e-1) * t23858 - F::new(0.16191709844559585492e0) * t23326 * t2466 + F::new(0.32383419689119170984e0) * t7208 * t8816;
    t29320
}
