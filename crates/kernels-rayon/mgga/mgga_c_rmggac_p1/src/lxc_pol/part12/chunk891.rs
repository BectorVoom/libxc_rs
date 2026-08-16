//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 891/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk891(t6355: f64, t7521: f64, t4905: f64, t8936: f64, t1240: f64, t236: f64, t3352: f64, t551: f64, t7230: f64, t34761: f64, t9153: f64, t16502: f64, t8516: f64) -> (f64, f64, f64, f64, f64) {
    let t39425 = t6355 * t7521;
    let t39427 = t8936 * t4905;
    let t39433 = t7230 * t3352 * t236 * t551 * t1240;
    let t39435 = t34761 * t9153;
    let t39437 = t8516 * t16502;
    (t39425, t39427, t39433, t39435, t39437)
}
