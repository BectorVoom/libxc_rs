//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 993/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk993<F: Float>(t1308: F, t6148: F, t19136: F, t6183: F, t19123: F, t6175: F, t13893: F, t9: F, t403: F, t19109: F, t3951: F, t963: F, t19119: F, t2075: F, t3961: F, t13472: F) -> (F, F, F, F, F, F, F, F) {
    let t20097 = t6148 * t1308;
    let t20104 = t6183 * t19136;
    let t20107 = t6175 * t19123;
    let t20110 = t9 * t13893;
    let t20111 = t20110 * t403;
    let t20112 = t20111 * t19109;
    let t20115 = t963 * t3951;
    let t20116 = t20115 * t403;
    let t20117 = t20116 * t19119;
    let t20120 = t2075 * t3961;
    let t20121 = t13472 * t20120;
    (t20097, t20104, t20107, t20111, t20112, t20116, t20117, t20121)
}
