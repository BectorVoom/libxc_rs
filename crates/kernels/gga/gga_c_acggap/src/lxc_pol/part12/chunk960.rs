//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 960/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk960<F: Float>(t1327: F, t361: F, t8888: F, t2060: F, t8630: F, t1413: F, t7685: F, t2001: F, t4535: F, t1441: F, t7614: F, t17972: F, t2068: F, t2263: F, t30984: F, t8649: F) -> (F, F, F, F, F, F, F) {
    let t35442 = t8888 * t361 * t1327;
    let t35445 = t2060 * t361 * t8630;
    let t35447 = t7685 * t1413;
    let t35449 = t2001 * t4535;
    let t35451 = t7614 * t1441;
    let t35454 = t2068 * t17972 * t2263;
    let t35456 = t30984 * t8649;
    (t35442, t35445, t35447, t35449, t35451, t35454, t35456)
}
