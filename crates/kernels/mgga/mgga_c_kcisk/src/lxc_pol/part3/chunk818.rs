//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 818/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk818<F: Float>(t12514: F, t830: F, t2885: F, t172: F, t849: F, t157: F, t2914: F, t2922: F, t119: F, t814: F, t298: F, t831: F) -> (F, F, F, F, F) {
    let t12586 = t12514 * t830;
    let t12588 = F::new(6.0) * t2885 * t12586;
    let t12589 = t172 * t849;
    let t12592 = t157 * t2914;
    let t12595 = t157 * t2922;
    let t12598 = t119 * t814;
    let t12601 = F::cast_from(0.71233333333333333334e-1_f64) * t298 * t12598 * t831;
    (t12588, t12589, t12592, t12595, t12601)
}
