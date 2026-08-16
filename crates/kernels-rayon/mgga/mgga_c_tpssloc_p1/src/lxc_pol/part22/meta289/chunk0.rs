//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1444/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1444(t13360: f64, t849: f64, t13176: f64, t842: f64, t1516: f64, t9601: f64, t1509: f64, t852: f64, t252: f64, t4233: f64, t4290: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13362 = 7.0_f64 / 576.0_f64 * t13360 * t849;
    let t13365 = t13176 * t842;
    let t13368 = t9601 * t1516;
    let t13380 = t852 * t1509;
    let t13384 = t252 * t4233;
    let t13390 = t808 * t4290;
    (t13362, t13365, t13368, t13380, t13384, t13390)
}
