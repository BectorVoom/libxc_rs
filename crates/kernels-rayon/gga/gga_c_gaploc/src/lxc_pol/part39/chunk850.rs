//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 850/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk850(t1: f64, t1559: f64, t544: f64, t986: f64, t10241: f64, t1359: f64, t1352: f64, t3690: f64, t3689: f64, t447: f64, t2366: f64, t475: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35204 = t544 * t1559 * t986 * t1;
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t38267 = t3690 * t1352;
    let t38271 = t3689 * t447;
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    (t35204, t35215, t35216, t38267, t38272, t38276)
}
