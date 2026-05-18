//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 873/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk873<F: Float>(t13121: F, t321: F, t1201: F, t13050: F, t13053: F, t13056: F, t13060: F, t13066: F, t13101: F, t13105: F, t3692: F, t3699: F, t3718: F, t3726: F) -> (F, F) {
    let t13123 = F::new(0.62182e-1) * t13121 * t321;
    let t13124 = -F::new(0.17544670192365612213e1) * t3692 * t3718 - t13050 + t13053 - t13056 + t13060 - F::new(0.51947267698127589899e2) * t3692 * t3726 + F::new(0.1038945353962551798e3) * t1201 * t13066 - F::new(0.58482233974552040708e0) * t1201 * t13101 - F::new(0.35089340384731224426e1) * t1201 * t13105 + F::new(0.35089340384731224426e1) * t3692 * t3699 - t13123;
    (t13123, t13124)
}
