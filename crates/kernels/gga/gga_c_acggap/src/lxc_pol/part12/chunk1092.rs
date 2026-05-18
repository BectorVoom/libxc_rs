//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1092/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1092<F: Float>(t1323: F, t361: F, t7436: F, t1327: F, t8888: F, t2060: F, t8630: F, t1413: F, t7685: F, t2001: F, t4535: F, t1441: F, t7614: F) -> (F, F, F, F, F, F) {
    let t35439 = t7436 * t361 * t1323;
    let t35442 = t8888 * t361 * t1327;
    let t35445 = t2060 * t361 * t8630;
    let t35447 = t7685 * t1413;
    let t35449 = t2001 * t4535;
    let t35451 = t7614 * t1441;
    (t35439, t35442, t35445, t35447, t35449, t35451)
}
