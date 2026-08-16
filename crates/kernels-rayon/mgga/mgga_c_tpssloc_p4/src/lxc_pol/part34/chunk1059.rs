//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1059/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1059(t1408: f64, t1530: f64, t25: f64, t5660: f64, t28: f64, t5527: f64, t23788: f64, t28248: f64, t1484: f64, t1649: f64, t5544: f64, t5664: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28459 = t1408 * t1530;
    let t28462 = t25 * t5660;
    let t28764 = t28 * t5527;
    let t28771 = t23788 * t28248;
    let t28774 = t1649 * t1484;
    let t28778 = t28 * t5544;
    let t28789 = t28 * t5664;
    (t28459, t28462, t28764, t28771, t28774, t28778, t28789)
}
