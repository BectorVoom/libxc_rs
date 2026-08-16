//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1924/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1924(t26418: f64, t6914: f64, t7736: f64, t80854: f64, t81064: f64, t22704: f64, t22705: f64, t26410: f64, t26432: f64, t6897: f64, t794: f64, t22642: f64, t22690: f64, t26395: f64) -> (f64, f64, f64, f64, f64) {
    let t90970 = t6914 * t26418;
    let t90980 = t81064 * t80854 * t7736;
    let t90983 = t22704 * t22705 * t26410;
    let t90987 = t6897 * t794 * t26432;
    let t90993 = t22642 * t22690 * t26395;
    (t90970, t90980, t90983, t90987, t90993)
}
