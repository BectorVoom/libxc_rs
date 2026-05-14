//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1159/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1159<F: Float>(t1299: F, t1566: F, t20: F, t2734: F, t14636: F, t79: F, t2736: F, t9511: F, t9523: F, t394: F, t4368: F, t14609: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32357 = t1566 * t1299;
    let t32358 = t32357 * t20;
    let t32359 = t2734 * t32358;
    let t32362 = t14636 * t79;
    let t32363 = t32362 * t2736;
    let t32366 = t9511 * t9523;
    let t32369 = t4368 * t394;
    let t32370 = t32369 * t20;
    let t32371 = t2734 * t32370;
    let t32376 = t14609 * t79;
    (t32357, t32358, t32359, t32362, t32363, t32366, t32369, t32370, t32371, t32376)
}
