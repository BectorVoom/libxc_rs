//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1058/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1058<F: Float>(t24216: F, t735: F, t1935: F, t23052: F, t642: F, t734: F, t2591: F, t7410: F, t23980: F, t7316: F, t17874: F, t23304: F, t7311: F, t7310: F, t23299: F, t5290: F) -> (F, F, F, F, F, F, F, F) {
    let t24217 = t735 * t24216;
    let t24218 = t1935 * t24217;
    let t24220 = t642 * t23052;
    let t24221 = t735 * t24220;
    let t24222 = t734 * t24221;
    let t24224 = t7410 * t2591;
    let t24226 = t7316 * t23980;
    let t24227 = t17874 * t24226;
    let t24229 = t7311 * t23304;
    let t24230 = t7310 * t24229;
    let t24232 = t5290 * t23299;
    (t24218, t24222, t24224, t24226, t24227, t24229, t24230, t24232)
}
