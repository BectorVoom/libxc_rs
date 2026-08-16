//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 800/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk800(t2049: f64, t5552: f64, t11709: f64, t11712: f64, t11715: f64, t11718: f64, t11721: f64, t11724: f64, t11728: f64, t11732: f64, t11736: f64, t11739: f64, t11742: f64, t11745: f64, t11747: f64, t11751: f64, t11754: f64, t11756: f64, t11758: f64, t11760: f64) -> (f64, f64) {
    let t12356 = t2049 * t5552;
    let t12377 = -0.1875e0_f64 * t11709 + 0.375e0_f64 * t11712 - 0.1875e0_f64 * t11715 + 0.80937499999999999999e-1_f64 * t11718 + 0.5625e0_f64 * t11721 - 0.80937499999999999999e-1_f64 * t11724 - 0.101171875e-1_f64 * t11728 + 0.5625e0_f64 * t11732 + 0.101171875e-1_f64 * t11736 + 0.625e-1_f64 * t11739 - 0.60703125e-1_f64 * t11742 + 0.303515625e-1_f64 * t11745 - 0.13489583333333333333e-1_f64 * t11747 + 0.625e-1_f64 * t11751 + 0.40468749999999999999e-1_f64 * t11754 - 0.40468749999999999999e-1_f64 * t11756 + 0.1875e0_f64 * t11758 - 0.40468749999999999999e-1_f64 * t11760;
    (t12356, t12377)
}
