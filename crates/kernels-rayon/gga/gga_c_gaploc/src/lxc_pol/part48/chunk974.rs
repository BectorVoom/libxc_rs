//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 974/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk974(t13368: f64, t4953: f64, t10345: f64, t10424: f64, t10497: f64, t10557: f64, t10615: f64, t13430: f64, t1424: f64, t1445: f64, t1562: f64, t204: f64, t2476: f64, t3338: f64, t34223: f64, t42001: f64, t46084: f64, t46414: f64, t46420: f64, t46422: f64, t46426: f64, t46432: f64, t46435: f64, t46447: f64, t46450: f64, t46457: f64, t46461: f64, t6710: f64, t6711: f64, t8097: f64) -> f64 {
    let t46463 = 0.62115540045351614476e2_f64 * t4953 * t13368;
    let t46464 = -0.39722766613167140743e-1_f64 * t46414 * t1424 - 0.11916829983950142223e0_f64 * t42001 + t46420 + t46422 + t46426 - 0.23005755572352449806e2_f64 * t6710 * t6711 * t46084 - t46432 + t46435 + 0.71500979903700853338e0_f64 * t10424 * t10497 + 0.92023022289409799224e1_f64 * t2476 * t204 * t46084 - 0.50050685932590597338e1_f64 * t10615 * t34223 + 0.85801175884441024008e1_f64 * t10557 * t10345 - t46447 - t46450 - 0.13803453343411469884e2_f64 * t4953 * t13430 - 0.13803453343411469884e2_f64 * t1562 * t1445 * t8097 * t3338 - 0.57514388930881124515e0_f64 * t46457 + t46461 - t46463;
    t46464
}
