//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1159/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1159(t225: f64, t26884: f64, t494: f64, t1210: f64, t8945: f64, t1248: f64, t1287: f64, t7638: f64, t487: f64, t7642: f64, t7644: f64, t3588: f64, t7660: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t26886 = t26884 * t225 * t494;
    let t26889 = t1210 * t8945;
    let t26891 = t7638 * t1248 * t1287;
    let t26894 = t7642 * t487;
    let t26895 = t26894 * t8945;
    let t26896 = t7644 * t1248;
    let t26897 = t26896 * t1287;
    let t26901 = t7660 * t3588 * t1287;
    (t26886, t26889, t26891, t26894, t26895, t26896, t26897, t26901)
}
