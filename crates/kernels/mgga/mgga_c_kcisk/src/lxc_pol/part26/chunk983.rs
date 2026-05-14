//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 983/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk983<F: Float>(t2083: F, t3539: F, t5601: F, t2075: F, t3544: F, t5703: F, t2191: F, t1175: F, t13148: F, t7736: F, t13153: F, t1364: F, t25446: F, t5895: F, t19330: F, t25465: F) -> (F, F, F, F, F, F, F) {
    let t26528 = t3539 * t5601 * t2083;
    let t26532 = t3544 * t2075 * t5703;
    let t26536 = t3544 * t5601 * t2191;
    let t26540 = t13148 * t7736 * t1175;
    let t26544 = t13153 * t7736 * t1364;
    let t26547 = t5895 * t25446;
    let t26550 = t19330 * t25465;
    (t26528, t26532, t26536, t26540, t26544, t26547, t26550)
}
