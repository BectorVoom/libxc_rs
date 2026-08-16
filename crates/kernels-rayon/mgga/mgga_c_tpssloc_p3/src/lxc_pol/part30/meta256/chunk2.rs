//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1160/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1160(t1934: f64, t6722: f64, t1933: f64, t40: f64, t1937: f64, t3: f64, t607: f64, t343: f64, t984: f64) -> (f64, f64, f64, f64, f64) {
    let t6723 = t6722 * t1934;
    let t6726 = t1933 * t40;
    let t6728 = 0.10093189023535097714e-3_f64 * t6726 * t1937;
    let t6729 = t3 * t607;
    let t6730 = t1933 * t6729;
    let t6733 = t984 * t343;
    (t6723, t6728, t6729, t6730, t6733)
}
