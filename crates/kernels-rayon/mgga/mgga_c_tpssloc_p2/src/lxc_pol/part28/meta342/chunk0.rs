//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1287/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1287(t12757: f64, t666: f64, t2358: f64, t4043: f64, t1444: f64, t2342: f64, t9384: f64, t2341: f64, t92: f64, t2219: f64, t659: f64, t2248: f64, t4049: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12758 = t12757 * t666;
    let t12761 = t4043 * t2358;
    let t12771 = t9384 * t1444 * t2342;
    let t12774 = t92 * t2341;
    let t12775 = t2219 * t659;
    let t12778 = t4049 * t2248;
    (t12758, t12761, t12771, t12774, t12775, t12778)
}
