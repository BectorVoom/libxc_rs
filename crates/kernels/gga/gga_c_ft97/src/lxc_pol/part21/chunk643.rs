//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 643/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk643<F: Float>(t15903: F, t446: F, t15768: F, t447: F, t15763: F, t1866: F, t358: F, t4436: F, t363: F, t7824: F, t432: F, t4458: F, t1564: F, t15752: F, t15756: F, t3281: F) -> (F, F, F, F, F, F, F, F, F) {
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
    let t15918 = t1564 * t15917;
    let t15919 = t446 * t15918;
    let t15921 = t447 * t15752;
    let t15922 = t446 * t15921;
    let t15924 = t447 * t15756;
    let t15925 = t3281 * t15924;
    (t15904, t15907, t15910, t15913, t15915, t15917, t15919, t15922, t15925)
}
