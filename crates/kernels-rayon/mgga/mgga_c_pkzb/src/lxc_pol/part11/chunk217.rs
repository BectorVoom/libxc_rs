//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 217/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk217(t702: f64, t703: f64, t650: f64, t657: f64) -> (f64, f64, f64) {
    let t704 = t702 * t703;
    let t707 = 0.92708333333333333333e-2_f64 * t650;
    let t709 = -t707 + 0.278125e-1_f64 * t657;
    (t704, t707, t709)
}
