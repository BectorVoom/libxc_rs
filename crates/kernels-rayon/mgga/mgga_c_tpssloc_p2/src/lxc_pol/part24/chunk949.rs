//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 949/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk949(t2862: f64, t931: f64, t932: f64, t2904: f64, t938: f64, t10524: f64, t951: f64, t10603: f64, t10629: f64, t315: f64, t10632: f64, t2853: f64, t923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10743 = t2862 * t931;
    let t10744 = t10743 * t932;
    let t10747 = t938 * t2904;
    let t10750 = t10524 * t951;
    let t10753 = t10603 * t951;
    let t10756 = t315 * t10629;
    let t10757 = t10524 * t10632;
    let t10760 = t2853 * t923;
    (t10743, t10744, t10747, t10750, t10753, t10756, t10757, t10760)
}
