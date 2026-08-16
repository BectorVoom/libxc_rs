//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1245/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1245(t33228: f64, t33354: f64, t33558: f64, t33625: f64, t3: f64, t1873: f64, t27254: f64, t24465: f64, t7769: f64, t7230: f64, t7467: f64, t16524: f64, t8657: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33627 = t33228 + t33354 + t33558 + t33625;
    let t33628 = t3 * t33627;
    let t33641 = 0.135e2_f64 * t27254 * t1873;
    let t33643 = 27.0_f64 * t24465 * t7769;
    let t33645 = 0.135e2_f64 * t7230 * t7467;
    let t33653 = 27.0_f64 * t16524 * t8657;
    (t33627, t33628, t33641, t33643, t33645, t33653)
}
