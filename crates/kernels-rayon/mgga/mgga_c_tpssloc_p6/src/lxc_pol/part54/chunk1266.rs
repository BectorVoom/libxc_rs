//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1266/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1266(t30643: f64, t6547: f64, t23109: f64, t23110: f64, t232: f64, t59: f64, t828: f64, t23062: f64, t30700: f64, t240: f64, t241: f64, t2627: f64, t812: f64) -> (f64, f64, f64, f64) {
    let t112760 = t6547 * t30643;
    let t112778 = t23109 * t23110 * t59 * t828 * t232;
    let t112784 = t23062 * t30700;
    let t112792 = t812 * t2627 * t240 * t241;
    (t112760, t112778, t112784, t112792)
}
