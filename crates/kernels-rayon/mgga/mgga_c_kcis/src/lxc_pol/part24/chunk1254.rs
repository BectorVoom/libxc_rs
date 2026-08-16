//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1254/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1254(t28912: f64, t9386: f64, t26671: f64, t2842: f64, t28911: f64, t100466: f64, t100474: f64, t100477: f64, t100480: f64, t100482: f64, t27028: f64, t27042: f64, t28146: f64, t28190: f64, t29127: f64, t5329: f64, t68901: f64, t7772: f64, t7788: f64) -> (f64, f64, f64) {
    let t100486 = t9386 * t28912;
    let t100489 = t2842 * t26671 * t28911;
    let t100491 = -0.69505208333333333334e-3_f64 * t7788 * t5329 * t27028 * t68901 - 0.34752604166666666667e-3_f64 * t7788 * t100466 - 0.46377350260416666667e-4_f64 * t7772 * t100466 - 0.46336805555555555556e-3_f64 * t28190 * t28146 + 0.69644166666666666666e-2_f64 * t100474 - 0.92858888888888888888e-2_f64 * t100477 - 0.23214722222222222222e-2_f64 * t100480 - 0.30945286961263020834e-5_f64 * t100482 - 0.12367293402777777778e-3_f64 * t27042 * t29127 + 0.12897067901234567901e-2_f64 * t100486 - 0.51588271604938271605e-2_f64 * t100489;
    (t100486, t100489, t100491)
}
