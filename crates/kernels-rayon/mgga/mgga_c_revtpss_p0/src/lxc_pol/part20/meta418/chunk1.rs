//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1561/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1561(t1145: f64, t141: f64, t43797: f64, t12327: f64, t3391: f64, t3399: f64, t12322: f64, t12343: f64, t43762: f64, t43769: f64, t43771: f64, t43773: f64, t43779: f64, t43781: f64, t43783: f64, t43785: f64, t43787: f64, t43791: f64, t43795: f64) -> (f64, f64, f64, f64) {
    let t43799 = t141 * t1145 * t43797;
    let t43802 = t12327 * t3391 * t3399;
    let t43804 = t12343 * t12322;
    let t43806 = -0.98115555555555555555e-1_f64 * t43762 - 0.8585111111111111111e-1_f64 * t43769 - 0.98115555555555555556e0_f64 * t43771 + 0.44152e0_f64 * t43773 + 0.44152e0_f64 * t43779 + 0.5519e0_f64 * t43781 + 0.11038e1_f64 * t43783 - 0.22076e0_f64 * t43785 - 0.132456e1_f64 * t43787 - 0.99342e0_f64 * t43791 + 0.198684e1_f64 * t43795 + 0.82785e-1_f64 * t43799 + 0.11651625e2_f64 * t43802 - 0.51785e1_f64 * t43804;
    (t43799, t43802, t43804, t43806)
}
