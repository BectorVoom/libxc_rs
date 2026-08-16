//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 765/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk765(t12509: f64, t625: f64, t11: f64, t12355: f64, t626: f64, t10581: f64, t10583: f64, t10585: f64, t12495: f64, t12499: f64, t12503: f64, t12507: f64, t5360: f64, t7269: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12510 = t625 * t12509;
    let t12511 = t11 * t12510;
    let t12513 = t626 * t12355;
    let t12514 = t625 * t12513;
    let t12515 = t11 * t12514;
    let t12517 = t5360 + 0.25188888888888888889e-2_f64 * t7269 - 0.12594444444444444445e-2_f64 * t10581 + 0.37783333333333333335e-2_f64 * t10583 - 0.18891666666666666667e-2_f64 * t10585 + 0.20990740740740740742e-2_f64 * t12495 - 0.75566666666666666669e-2_f64 * t12499 + 0.37783333333333333335e-2_f64 * t12503 + 0.11335e-1_f64 * t12507 - 0.11335e-1_f64 * t12511 + 0.18891666666666666667e-2_f64 * t12515;
    (t12510, t12511, t12513, t12514, t12515, t12517)
}
