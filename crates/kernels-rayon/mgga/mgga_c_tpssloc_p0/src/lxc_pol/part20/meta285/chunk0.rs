//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1480/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1480(t10743: f64, t932: f64, t2904: f64, t938: f64, t10524: f64, t951: f64, t10603: f64, t10629: f64, t315: f64) -> (f64, f64, f64, f64, f64) {
    let t10744 = t10743 * t932;
    let t10747 = t938 * t2904;
    let t10750 = t10524 * t951;
    let t10753 = t10603 * t951;
    let t10756 = t315 * t10629;
    (t10744, t10747, t10750, t10753, t10756)
}
