//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1588/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1588<F: Float>(t19390: F, t19434: F, t20187: F, t20228: F, t1100: F, t1102: F, t19143: F, t19145: F, t19149: F, t19152: F, t19153: F, t19252: F, t19258: F, t19315: F, t19317: F, t19320: F, t19323: F, t19326: F, t19329: F, t19333: F, t19337: F, t19470: F, t19473: F, t19475: F, t198: F, t336: F, t5019: F, t5023: F, t5024: F) -> (F, F) {
    let t20230 = t19390 + t19434 + t20187 + t20228;
    let t20234 = t1102 * t198 * t20230 * t336 - t1100 * t19153 * t5023 - F::cast_from(2.0_f64) * t5019 * t5023 * t5024 + t19143 - t19145 + t19149 + t19152 + t19252 + t19258 - t19315 + t19317 + t19320 - t19323 - t19326 - t19329 + t19333 + t19337 - t19470 - t19473 - t19475;
    (t20230, t20234)
}
