//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1222/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1222(t3508: f64, t6218: f64, t11721: f64, t6224: f64, t11818: f64, t1213: f64, t248: f64, t6219: f64, t3036: f64, t6163: f64, t3500: f64, t3503: f64) -> (f64, f64, f64, f64, f64) {
    let t65464 = t6218 * t3508;
    let t65474 = t6224 * t11721;
    let t65528 = t1213 * t248 * t11818 * t6219;
    let t65539 = t6163 * t3036;
    let t65541 = t3500 * t3503 * t65539;
    (t65464, t65474, t65528, t65539, t65541)
}
