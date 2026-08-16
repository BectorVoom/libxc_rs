//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 766/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk766(t358: f64, t4495: f64, t363: f64, t1564: f64, t446: f64, t15768: f64, t447: f64, t15763: f64, t1866: f64, t4436: f64, t7824: f64, t432: f64, t4458: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15901 = t4495 * t358;
    let t15902 = t15901 * t363;
    let t15903 = t1564 * t15902;
    let t15904 = t446 * t15903;
    let t15906 = t447 * t15768;
    let t15907 = t446 * t15906;
    let t15909 = t1866 * t15763;
    let t15910 = t446 * t15909;
    let t15912 = t4436 * t358;
    let t15913 = t15912 * t363;
    let t15914 = t7824 * t15913;
    let t15915 = t446 * t15914;
    let t15917 = t4458 * t432;
    (t15902, t15904, t15907, t15910, t15913, t15915, t15917)
}
