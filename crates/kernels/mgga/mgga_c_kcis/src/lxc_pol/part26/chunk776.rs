//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 776/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk776<F: Float>(t88: F, t8998: F, t138: F, t86: F, t221: F, t2720: F, t2725: F, t2729: F, t8765: F, t889: F, t8913: F, t8932: F, t8934: F, t8937: F, t8939: F, t8944: F, t8949: F, t8957: F, t8961: F, t8965: F) -> (F, F, F) {
    let t8999 = t88 * t8998;
    let t9001 = t86 * t8999 * t138;
    let t9004 = -F::cast_from(0.65001222222222222219e-1_f64) * t8932 - F::cast_from(0.55715333333333333331e-1_f64) * t8934 - F::cast_from(0.2089325e-1_f64) * t8937 - F::cast_from(0.200175e0_f64) * t8939 * t889 - F::cast_from(0.178244852896875e-2_f64) * t8944 * t8765 + F::cast_from(0.41786499999999999999e-1_f64) * t8949 + F::cast_from(0.200175e0_f64) * t2720 * t2729 - F::cast_from(0.2671335375e-1_f64) * t2725 * t8765 + F::cast_from(0.65001222222222222219e-1_f64) * t8957 - F::cast_from(0.72223580246913580243e-1_f64) * t8961 - F::cast_from(0.27857666666666666666e-1_f64) * t8965 + F::cast_from(0.69644166666666666665e-2_f64) * t9001 + t8913 * t221;
    (t8999, t9001, t9004)
}
