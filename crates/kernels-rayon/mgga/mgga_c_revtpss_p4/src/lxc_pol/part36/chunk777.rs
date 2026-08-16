//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 777/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk777(t7637: f64, t8197: f64, t1774: f64, t2142: f64, t1811: f64, t2148: f64, t1828: f64, t7652: f64, t1287: f64, t1794: f64, t7660: f64, t2150: f64, t473: f64, t8190: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8198 = t7637 * t8197;
    let t8201 = t2142 * t1774;
    let t8202 = t7637 * t8201;
    let t8205 = t2148 * t1811;
    let t8208 = t2142 * t1828;
    let t8209 = t7652 * t8208;
    let t8213 = t7660 * t1794 * t1287;
    let t8217 = t2150 * t473 * t8190;
    (t8198, t8201, t8202, t8205, t8208, t8209, t8213, t8217)
}
