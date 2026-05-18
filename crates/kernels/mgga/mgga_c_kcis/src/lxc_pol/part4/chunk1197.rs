//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1197/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1197<F: Float>(t10893: F, t10898: F, t10936: F, t13798: F, t13801: F, t13805: F, t13974: F, t15304: F, t15307: F, t15310: F, t15317: F, t15320: F, t15323: F, t3550: F, t3575: F, t3586: F, t3592: F, t5216: F, t5238: F) -> F {
    let t15326 = -F::new(0.19751789702565206229e-1) * t13974 + t13798 + t13801 - t13805 - F::new(0.11696446794910408142e1) * t15304 * t3586 + F::new(6.0) * t3575 * t15307 + F::new(0.35089340384731224426e1) * t3592 * t15310 - F::new(4.0) * t10936 * t5216 + F::new(0.64329366355741395948e2) * t10893 * t5238 - F::new(4.0) * t3550 * t15317 - F::new(2.0) * t3550 * t15320 - F::new(0.19298809906722418785e3) * t10898 * t15323;
    t15326
}
