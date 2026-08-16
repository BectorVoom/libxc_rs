//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1153/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1153(t11239: f64, t3143: f64, t342: f64, t12051: f64, t3154: f64, t12048: f64, t1071: f64, t3151: f64, t3304: f64, t3318: f64, t11687: f64, t4998: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12077 = t11239 * t3143;
    let t12078 = t342 * t12077;
    let t12079 = t12051 * t3154;
    let t12080 = t12048 * t12079;
    let t12085 = t1071 * t3151;
    let t12086 = t12085 * t3304;
    let t12089 = t12085 * t3318;
    let t12094 = t11687 * t4998;
    (t12077, t12078, t12079, t12080, t12086, t12089, t12094)
}
