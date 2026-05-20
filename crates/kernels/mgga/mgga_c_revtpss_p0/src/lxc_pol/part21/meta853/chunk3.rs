//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3215/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3215<F: Float>(t127: F, t17693: F, t17695: F, t5268: F, t17708: F, t45779: F, t13089: F, t5391: F, t13085: F, t5381: F, t1284: F, t17306: F, t3624: F) -> (F, F, F, F, F, F) {
    let t59391 = t17693 * t127 * t5268 * t17695;
    let t59401 = t45779 * t17708;
    let t59404 = t5391 * t13089;
    let t59406 = t5381 * t13085;
    let t59408 = t5381 * t13089;
    let t59411 = t17306 * t1284 * t3624;
    (t59391, t59401, t59404, t59406, t59408, t59411)
}
