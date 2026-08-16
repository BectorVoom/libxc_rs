//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 910/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk910(t2010: f64, t38835: f64, t8465: f64, t2415: f64, t38820: f64, t7349: f64, t2329: f64, t38973: f64, t118: f64, t128: f64, t2001: f64, t6261: f64, t675: f64) -> (f64, f64, f64, f64) {
    let t45152 = t2010 * t8465 * t38835;
    let t45155 = t7349 * t2415 * t38820;
    let t45158 = t38973 * t2329;
    let t45163 = t675 * t2001 * t118 * t128 * t6261;
    (t45152, t45155, t45158, t45163)
}
