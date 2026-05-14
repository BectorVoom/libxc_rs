//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1349/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1349<F: Float>(t10024: F, t5531: F, t17787: F, t33071: F, t34288: F, t5339: F, t2041: F, t34608: F, t1907: F, t34286: F, t1957: F, t11701: F, t34303: F, t34300: F, t47024: F, t5217: F, t9963: F) -> (F, F, F, F, F, F, F, F) {
    let t117552 = t10024 * t5531;
    let t117556 = 4.0 * t33071 * t17787;
    let t117557 = t34288 * t5339;
    let t117560 = t34608 * t2041;
    let t117563 = t34286 * t1907;
    let t117565 = 2.0 * t117563 * t1957;
    let t117568 = 12.0 * t11701 * t34303 * t1957;
    let t117574 = 12.0 * t47024 * t34300;
    let t117580 = t9963 * t5217;
    (t117552, t117556, t117557, t117560, t117565, t117568, t117574, t117580)
}
