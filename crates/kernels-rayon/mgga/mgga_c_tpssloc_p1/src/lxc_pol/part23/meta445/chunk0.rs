//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1290/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1290(t118: f64, t20416: f64, t3739: f64, t794: f64, t16094: f64, t16095: f64, t6347: f64, t686: f64, t213: f64, t20582: f64, t40021: f64, t20356: f64, t40412: f64) -> (f64, f64, f64, f64, f64) {
    let t74702 = t3739 * t118 * t794 * t20416;
    let t74724 = t16094 * t686 * t16095 * t6347;
    let t74726 = t213 * t20416;
    let t74741 = t40021 * t20582;
    let t74745 = t40412 * t118 * t794 * t20356;
    (t74702, t74724, t74726, t74741, t74745)
}
