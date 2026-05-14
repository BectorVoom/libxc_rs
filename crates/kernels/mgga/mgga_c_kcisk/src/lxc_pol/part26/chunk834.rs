//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 834/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk834<F: Float>(t1474: F, t979: F, t140: F, t1477: F, t430: F, t1390: F, t1402: F, t3532: F, t4214: F, t469: F, t1511: F, t492: F, t1414: F, t1555: F, t524: F, t4349: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14439 = t979 * t1474;
    let t14469 = t140 * t430 * t1477;
    let t14475 = t1402 * t1390;
    let t14491 = t1402 * t3532;
    let t14581 = t4214 * t469;
    let t14591 = t492 * t1511;
    let t14592 = t1414 * t14591;
    let t14607 = t1555 * t1555;
    let t14608 = 1.0 / t14607;
    let t14609 = t524 * t14608;
    let t14612 = 1.0 / t4349 / t544;
    (t14439, t14469, t14475, t14491, t14581, t14592, t14607, t14608, t14609, t14612)
}
