//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1237/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1237(t2075: f64, t7467: f64, t652: f64, t1458: f64, t8595: f64, t2095: f64, t33136: f64, t1983: f64, t1873: f64, t27254: f64, t24465: f64, t7769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t33617 = t2075 * t7467;
    let t33619 = 2.0_f64 * t652 * t33617;
    let t33620 = t8595 * t1458;
    let t33622 = 2.0_f64 * t652 * t33620;
    let t33623 = t2095 * t33136;
    let t33624 = t1983 * t33623;
    let t33641 = 0.135e2_f64 * t27254 * t1873;
    let t33643 = 27.0_f64 * t24465 * t7769;
    (t33617, t33619, t33620, t33622, t33623, t33624, t33641, t33643)
}
