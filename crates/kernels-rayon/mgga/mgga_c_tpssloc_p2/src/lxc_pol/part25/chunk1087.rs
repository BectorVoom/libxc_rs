//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1087/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1087(t22797: f64, t3770: f64, t12313: f64, t6916: f64, t213: f64, t6924: f64, t9223: f64, t6928: f64, t22804: f64, t22808: f64, t12012: f64, t1998: f64, t236: f64, t6926: f64) -> (f64, f64, f64, f64, f64) {
    let t80761 = t22797 * t3770;
    let t80763 = t6916 * t12313;
    let t80766 = t9223 * t6924 * t213;
    let t80767 = t80766 * t6928;
    let t80769 = t22804 * t22808;
    let t80773 = t6926 * t1998 * t236 * t12012;
    (t80761, t80763, t80767, t80769, t80773)
}
