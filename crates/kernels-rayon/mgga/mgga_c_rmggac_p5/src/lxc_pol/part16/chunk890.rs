//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 890/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk890(t38530: f64, t9153: f64, t2281: f64, t34975: f64, t35039: f64, t8455: f64, t14237: f64, t16503: f64, t9157: f64, t38523: f64, t9163: f64, t34962: f64, t9151: f64) -> (f64, f64, f64, f64, f64) {
    let t44755 = t38530 * t9153;
    let t44759 = t34975 * t35039 * t2281 * t8455;
    let t44763 = t16503 * t14237 * t2281 * t9157;
    let t44767 = t16503 * t35039 * t38523 * t9163;
    let t44771 = t16503 * t34962 * t2281 * t9151;
    (t44755, t44759, t44763, t44767, t44771)
}
