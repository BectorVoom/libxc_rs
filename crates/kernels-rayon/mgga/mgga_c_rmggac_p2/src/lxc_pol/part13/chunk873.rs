//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 873/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk873(t1986: f64, t5160: f64, t675: f64, t2191: f64, t8587: f64, t26857: f64, t7518: f64, t6355: f64, t7521: f64, t1240: f64, t236: f64, t3352: f64, t551: f64, t7230: f64) -> (f64, f64, f64, f64, f64) {
    let t39418 = t675 * t1986 * t5160;
    let t39420 = t2191 * t8587;
    let t39423 = t26857 * t7518;
    let t39425 = t6355 * t7521;
    let t39433 = t7230 * t3352 * t236 * t551 * t1240;
    (t39418, t39420, t39423, t39425, t39433)
}
