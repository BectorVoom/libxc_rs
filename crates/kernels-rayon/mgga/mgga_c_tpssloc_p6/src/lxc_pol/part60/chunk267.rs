//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 267/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk267(t1598: f64, t343: f64, t974: f64, t1593: f64, t971: f64, t973: f64, t381: f64, t1409: f64, t998: f64, t225: f64, t68: f64, t369: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1599 = t1598 * t343;
    let t1600 = t974 * t1599;
    let t1603 = t971 + 0.27777777777777777777e-3_f64 * t973 * t1593 - 0.83333333333333333332e-3_f64 * t973 * t1600;
    let t1604 = t1603 * t381;
    let t1606 = t998 * t1409;
    let t1607 = t974 * t1606;
    let t1610 = t1603 * t225;
    let t1611 = t1610 * t68;
    let t1612 = t1611 * t369;
    (t1599, t1603, t1604, t1606, t1607, t1610, t1611, t1612)
}
