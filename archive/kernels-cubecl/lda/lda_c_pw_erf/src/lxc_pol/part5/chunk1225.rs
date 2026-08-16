//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1225/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1225<F: Float>(t2146: F, t6389: F, t10419: F, t17979: F, t17981: F, t17983: F, t22084: F, t22086: F, t22088: F, t22093: F, t22098: F, t22102: F, t22107: F, t22109: F, t22111: F) -> (F, F, F, F, F, F) {
    let t22113 = F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t2146 * t6389;
    let t22114 = F::cast_from(32.0_f64) / F::cast_from(405.0_f64) * t10419;
    let t22115 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17979;
    let t22116 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17981;
    let t22117 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t17983;
    let t22118 = -t22084 + t22086 + t22088 + t22093 - t22098 + t22102 - t22107 - t22109 + t22111 - t22113 + t22114 - t22115 - t22116 + t22117;
    (t22113, t22114, t22115, t22116, t22117, t22118)
}
