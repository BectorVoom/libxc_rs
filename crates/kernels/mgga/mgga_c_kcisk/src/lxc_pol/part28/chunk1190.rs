//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1190/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1190<F: Float>(t34049: F, t34070: F, t34111: F, t34143: F, t34185: F, t34215: F, t34251: F, t34283: F, t752: F, t1907: F, t9963: F, t1957: F, t2594: F, t33068: F, t33071: F, t7296: F) -> (F, F, F, F, F, F) {
    let t34286 = t34049 + t34070 + t34111 + t34143 + t34185 + t34215 + t34251 + t34283;
    let t34287 = t34286 * t752;
    let t34288 = t9963 * t1907;
    let t34289 = t34288 * t1957;
    let t34290 = t33068 * t2594;
    let t34292 = 2.0 * t33071 * t7296;
    (t34286, t34287, t34288, t34289, t34290, t34292)
}
