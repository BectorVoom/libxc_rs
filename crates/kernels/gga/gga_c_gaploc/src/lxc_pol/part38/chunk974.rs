//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 974/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk974<F: Float>(t13368: F, t4953: F, t10345: F, t10424: F, t10497: F, t10557: F, t10615: F, t13430: F, t1424: F, t1445: F, t1562: F, t204: F, t2476: F, t3338: F, t34223: F, t42001: F, t46084: F, t46414: F, t46420: F, t46422: F, t46426: F, t46432: F, t46435: F, t46447: F, t46450: F, t46457: F, t46461: F, t6710: F, t6711: F, t8097: F) -> F {
    let t46463 = F::cast_from(0.62115540045351614476e2_f64) * t4953 * t13368;
    let t46464 = -F::cast_from(0.39722766613167140743e-1_f64) * t46414 * t1424 - F::cast_from(0.11916829983950142223e0_f64) * t42001 + t46420 + t46422 + t46426 - F::cast_from(0.23005755572352449806e2_f64) * t6710 * t6711 * t46084 - t46432 + t46435 + F::cast_from(0.71500979903700853338e0_f64) * t10424 * t10497 + F::cast_from(0.92023022289409799224e1_f64) * t2476 * t204 * t46084 - F::cast_from(0.50050685932590597338e1_f64) * t10615 * t34223 + F::cast_from(0.85801175884441024008e1_f64) * t10557 * t10345 - t46447 - t46450 - F::cast_from(0.13803453343411469884e2_f64) * t4953 * t13430 - F::cast_from(0.13803453343411469884e2_f64) * t1562 * t1445 * t8097 * t3338 - F::cast_from(0.57514388930881124515e0_f64) * t46457 + t46461 - t46463;
    t46464
}
