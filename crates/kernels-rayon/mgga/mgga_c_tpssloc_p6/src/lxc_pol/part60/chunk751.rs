//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 751/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk751(t2122: f64, t27381: f64, t24574: f64, t8003: f64, t6686: f64, t8020: f64, t1751: f64, t7284: f64, t8067: f64, t477: f64, t1419: f64, t6794: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t27382 = t2122 * t27381;
    let t27401 = t24574 * t8003;
    let t27406 = t8020 * t6686;
    let t27426 = t7284 * t1751;
    let t27451 = t24574 * t8067;
    let t27460 = t477 * t1751;
    let t27505 = t1419 * t6794;
    (t27382, t27401, t27406, t27426, t27451, t27460, t27505)
}
