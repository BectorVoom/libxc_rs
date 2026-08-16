//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1269/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1269(t19390: f64, t19434: f64, t20187: f64, t20228: f64, t1100: f64, t1102: f64, t19143: f64, t19145: f64, t19149: f64, t19152: f64, t19153: f64, t19252: f64, t19258: f64, t19315: f64, t19317: f64, t19320: f64, t19323: f64, t19326: f64, t19329: f64, t19333: f64, t19337: f64, t19470: f64, t19473: f64, t19475: f64, t198: f64, t336: f64, t5019: f64, t5023: f64, t5024: f64) -> f64 {
    let t20230 = t19390 + t19434 + t20187 + t20228;
    let t20234 = t1102 * t198 * t20230 * t336 - t1100 * t19153 * t5023 - 2.0_f64 * t5019 * t5023 * t5024 + t19143 - t19145 + t19149 + t19152 + t19252 + t19258 - t19315 + t19317 + t19320 - t19323 - t19326 - t19329 + t19333 + t19337 - t19470 - t19473 - t19475;
    t20234
}
