//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 989/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk989(t2156: f64, t2993: f64, t1306: f64, t135: f64, t273: f64, t7314: f64, t7413: f64, t7415: f64, t7417: f64, t7446: f64, t7485: f64, t7491: f64, t7493: f64, t7504: f64, t7552: f64, t7554: f64, t7557: f64, t7559: f64, t7562: f64, t7564: f64, t7566: f64, t7570: f64, t7573: f64, t7888: f64, t803: f64, t805: f64) -> (f64, f64) {
    let t7892 = t2993 * t2156;
    let t7896 = t135 * t273 * t7888 * t805 - 2.0_f64 * t1306 * t7892 * t803 + t7314 + t7413 + t7415 + t7417 + t7446 - t7485 + t7491 + t7493 - t7504 - t7552 + t7554 + t7557 - t7559 - t7562 - t7564 + t7566 - t7570 - t7573;
    (t7892, t7896)
}
