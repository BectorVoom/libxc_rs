//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 673/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk673(t6579: f64, t1878: f64, t229: f64, t805: f64, t1891: f64, t2230: f64, t213: f64, t1895: f64, t202: f64, t243: f64, t598: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6580 = 7.0_f64 / 288.0_f64 * t6579;
    let t6581 = t1878 * t229;
    let t6582 = t6581 * t805;
    let t6584 = t2230 * t1891;
    let t6585 = t6584 * t213;
    let t6586 = t6585 * t1895;
    let t6587 = 0.14130464632949136799e-2_f64 * t6586;
    let t6589 = 1.0_f64 / t243 / t202;
    let t6590 = t598 * t6589;
    let t6591 = t6590 * t213;
    (t6580, t6581, t6582, t6584, t6585, t6587, t6589, t6590, t6591)
}
