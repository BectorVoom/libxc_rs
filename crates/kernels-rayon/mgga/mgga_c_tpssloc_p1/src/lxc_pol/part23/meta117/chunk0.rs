//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 603/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk603(t1409: f64, t3242: f64, t3247: f64, t1098: f64, t1657: f64, t1661: f64, t3270: f64, t3287: f64, t1667: f64, t699: f64, t1128: f64, t1675: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4723 = t3242 * t1409;
    let t4728 = t3247 * t1409;
    let t4740 = t1657 * t1098;
    let t4748 = t3270 * t1661;
    let t4764 = t3287 * t1661;
    let t4770 = t699 * t1667;
    let t4797 = t1675 * t1128;
    (t4723, t4728, t4740, t4748, t4764, t4770, t4797)
}
