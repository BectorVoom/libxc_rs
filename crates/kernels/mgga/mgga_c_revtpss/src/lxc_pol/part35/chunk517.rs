//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 517/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk517<F: Float>(t5883: F, t94: F, t1518: F, t1843: F, t1513: F, t2339: F, t1504: F, t2349: F, t100: F, t5823: F, t1479: F, t1509: F, t2357: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t97: F, tau1: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t5884 = t94 * t5883;
    let t5887 = t1843 * t1518;
    let t5891 = t1513 * t1513;
    let t5892 = t2339 * t5891;
    let t5895 = t1504 * t1504;
    let t5896 = t2349 * t5895;
    let t5899 = t100 * t5823;
    let t5902 = tau1 * t1479;
    let t5907 = t1509 * t1509;
    let t5908 = t2357 * t5907;
    let t5911 = -t5823;
    let t5912 = t108 * t5911;
    let t5915 = 10.0 / 9.0 * t97 * t5896 + 5.0 / 3.0 * t97 * t5899 + 40.0 / 9.0 * t5902 * t109 - 50.0 / 9.0 * t1507 * t1510 + 10.0 / 9.0 * t105 * t5908 + 5.0 / 3.0 * t105 * t5912;
    (t5884, t5887, t5891, t5892, t5895, t5902, t5907, t5908, t5911, t5912, t5915)
}
