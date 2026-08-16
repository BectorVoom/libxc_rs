//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1353/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1353(t1011: f64, t3493: f64, t225: f64, t24698: f64, t11720: f64, t2144: f64, t1193: f64, t24811: f64, t24817: f64, t24660: f64, t7319: f64, t24667: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85827 = t3493 * t1011;
    let t85832 = t24698 * t225;
    let t85836 = t2144 * t11720;
    let t85853 = t24811 * t1193;
    let t85854 = t85853 * t24817;
    let t85859 = t7319 * t24660;
    let t85863 = t7319 * t24667;
    (t85827, t85832, t85836, t85853, t85854, t85859, t85863)
}
