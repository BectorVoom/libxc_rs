//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 776/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk776(t88: f64, t8998: f64, t138: f64, t86: f64, t221: f64, t2720: f64, t2725: f64, t2729: f64, t8765: f64, t889: f64, t8913: f64, t8932: f64, t8934: f64, t8937: f64, t8939: f64, t8944: f64, t8949: f64, t8957: f64, t8961: f64, t8965: f64) -> (f64, f64, f64) {
    let t8999 = t88 * t8998;
    let t9001 = t86 * t8999 * t138;
    let t9004 = -0.65001222222222222219e-1_f64 * t8932 - 0.55715333333333333331e-1_f64 * t8934 - 0.2089325e-1_f64 * t8937 - 0.200175e0_f64 * t8939 * t889 - 0.178244852896875e-2_f64 * t8944 * t8765 + 0.41786499999999999999e-1_f64 * t8949 + 0.200175e0_f64 * t2720 * t2729 - 0.2671335375e-1_f64 * t2725 * t8765 + 0.65001222222222222219e-1_f64 * t8957 - 0.72223580246913580243e-1_f64 * t8961 - 0.27857666666666666666e-1_f64 * t8965 + 0.69644166666666666665e-2_f64 * t9001 + t8913 * t221;
    (t8999, t9001, t9004)
}
