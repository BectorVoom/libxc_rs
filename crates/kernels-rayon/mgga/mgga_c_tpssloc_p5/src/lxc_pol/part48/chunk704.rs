//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 704/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk704(t16312: f64, t550: f64, t1339: f64, t22827: f64, t242: f64, t6943: f64, t1336: f64, t3809: f64, t2002: f64, t3773: f64, t559: f64, t1878: f64, t557: f64) -> (f64, f64, f64, f64, f64) {
    let t22828 = t16312 * t550;
    let t22829 = t1339 * t22828;
    let t22830 = t22827 * t22829;
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    let t22834 = t22833 * t3809;
    let t22836 = t3773 * t2002;
    let t22837 = t22836 * t559;
    let t22839 = t1878 * t557;
    (t22828, t22830, t22834, t22837, t22839)
}
