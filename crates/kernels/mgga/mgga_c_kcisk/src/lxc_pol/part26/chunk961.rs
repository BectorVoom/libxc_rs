//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 961/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk961<F: Float>(t1248: F, t1249: F, t25441: F, t25450: F, t6074: F, t25432: F, t25437: F, t4065: F, t1224: F, t13528: F, t7736: F, t13538: F, t25465: F, t25446: F, t4013: F, t20310: F, t25469: F) -> (F, F, F, F, F, F, F, F) {
    let t26119 = t1248 * t1249 * t25441;
    let t26122 = t1248 * t6074 * t25450;
    let t26126 = t1248 * t1249 * t25432;
    let t26130 = t1248 * t4065 * t25437;
    let t26138 = t1224 * t13528 * t7736;
    let t26141 = t1224 * t13538 * t25465;
    let t26144 = t1224 * t4013 * t25446;
    let t26147 = t1224 * t20310 * t25469;
    (t26119, t26122, t26126, t26130, t26138, t26141, t26144, t26147)
}
