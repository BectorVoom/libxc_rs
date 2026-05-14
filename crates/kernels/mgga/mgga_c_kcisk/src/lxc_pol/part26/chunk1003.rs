//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1003/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1003<F: Float>(t3512: F, t8180: F, t1339: F, t1440: F, t8072: F, t1341: F, t13330: F, t1411: F, t1446: F, t8163: F, t415: F, t1333: F, t8164: F, t2075: F, t6001: F, t13377: F) -> (F, F, F, F, F, F, F) {
    let t26924 = t3512 * t8180;
    let t26925 = t1339 * t26924;
    let t26927 = t8072 * t1440;
    let t26928 = t1341 * t26927;
    let t26929 = t13330 * t26928;
    let t26930 = t1411 * t26929;
    let t26933 = t8163 * t1446;
    let t26934 = t415 * t26933;
    let t26936 = t1333 * t8164;
    let t26940 = t2075 * t6001;
    let t26941 = t13377 * t26940;
    (t26925, t26927, t26930, t26934, t26936, t26940, t26941)
}
