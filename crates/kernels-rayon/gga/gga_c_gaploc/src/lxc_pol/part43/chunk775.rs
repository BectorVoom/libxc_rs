//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 775/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk775(t10241: f64, t1359: f64, t544: f64, t1352: f64, t3690: f64, t3689: f64, t447: f64, t2366: f64, t475: f64, t6508: f64, t12000: f64, t158: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35215 = t1359 * t10241;
    let t35216 = t544 * t35215;
    let t38267 = t3690 * t1352;
    let t38271 = t3689 * t447;
    let t38272 = t2366 * t38271;
    let t38276 = t3689 * t475;
    let t38277 = t6508 * t38276;
    let t38281 = t2366 * t38276;
    let t38285 = t158 * t12000;
    (t35215, t35216, t38267, t38272, t38277, t38281, t38285)
}
