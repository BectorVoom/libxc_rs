//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 309/1444 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk309(t350: f64, t55: f64, t95: f64, t367: f64, t4: f64, t44: f64, t382: f64, t79: f64, t373: f64, t51: f64, t379: f64, t381: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1216 = t350 * t95 * t55;
    let t1218 = 0.24415406715670879921e-3_f64 * t367 * t1216;
    let t1219 = t44 * t4;
    let t1220 = t79 * t382;
    let t1222 = 0.10843580882781524214e-1_f64 * t1219 * t1220;
    let t1223 = t373 * t51;
    let t1224 = 1.0_f64 / t1223;
    let t1225 = t379 * t379;
    let t1227 = t1224 * t1225 * t381;
    (t1216, t1218, t1220, t1222, t1224, t1225, t1227)
}
