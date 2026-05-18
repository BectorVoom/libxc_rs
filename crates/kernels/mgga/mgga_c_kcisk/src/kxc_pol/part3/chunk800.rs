//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 800/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk800<F: Float>(t2049: F, t5552: F, t11709: F, t11712: F, t11715: F, t11718: F, t11721: F, t11724: F, t11728: F, t11732: F, t11736: F, t11739: F, t11742: F, t11745: F, t11747: F, t11751: F, t11754: F, t11756: F, t11758: F, t11760: F) -> (F, F) {
    let t12356 = t2049 * t5552;
    let t12377 = -F::new(0.1875e0) * t11709 + F::new(0.375e0) * t11712 - F::new(0.1875e0) * t11715 + F::new(0.80937499999999999999e-1) * t11718 + F::new(0.5625e0) * t11721 - F::new(0.80937499999999999999e-1) * t11724 - F::new(0.101171875e-1) * t11728 + F::new(0.5625e0) * t11732 + F::new(0.101171875e-1) * t11736 + F::new(0.625e-1) * t11739 - F::new(0.60703125e-1) * t11742 + F::new(0.303515625e-1) * t11745 - F::new(0.13489583333333333333e-1) * t11747 + F::new(0.625e-1) * t11751 + F::new(0.40468749999999999999e-1) * t11754 - F::new(0.40468749999999999999e-1) * t11756 + F::new(0.1875e0) * t11758 - F::new(0.40468749999999999999e-1) * t11760;
    (t12356, t12377)
}
