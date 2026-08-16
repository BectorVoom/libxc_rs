//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 688/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk688(t6976: f64, t7736: f64, t1992: f64, t1834: f64, t1998: f64, t214: f64, t1985: f64, t2031: f64, t7445: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7737 = t6976 * t7736;
    let t7738 = t1992 * t7737;
    let t7740 = t1998 * t1834;
    let t7741 = t214 * t7740;
    let t7742 = t1985 * t7741;
    let t7782 = t2031 * t7445;
    (t7737, t7738, t7740, t7741, t7742, t7782)
}
