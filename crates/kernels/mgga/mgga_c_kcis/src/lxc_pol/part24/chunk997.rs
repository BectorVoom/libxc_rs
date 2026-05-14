//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 997/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk997<F: Float>(t2189: F, t6638: F, t10498: F, t1820: F, t8081: F, t3330: F, t6735: F, t377: F, t6681: F, t28059: F, t8069: F, t28045: F, t8072: F, t5047: F, t6486: F, t26896: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t29036 = t2189 * t6638;
    let t29038 = 6.0 * t10498 * t29036;
    let t29039 = t8081 * t1820;
    let t29041 = 4.0 * t3330 * t29039;
    let t29042 = t2189 * t6735;
    let t29044 = 2.0 * t3330 * t29042;
    let t29045 = t6681 * t377;
    let t29047 = t28059 * t8069;
    let t29049 = t28045 * t8072;
    let t29051 = t5047 * t6486;
    let t29052 = t26896 * t29051;
    (t29036, t29038, t29039, t29041, t29042, t29044, t29045, t29047, t29049, t29051, t29052)
}
