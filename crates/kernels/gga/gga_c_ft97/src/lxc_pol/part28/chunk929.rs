//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 929/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk929<F: Float>(t144826: F, t144888: F, t144943: F, t144981: F, t145034: F, t145603: F, t145665: F, t145701: F, t488: F, t22943: F, t25598: F, t31995: F, t6414: F, t1286: F, t136116: F, t137476: F, t137497: F, t144765: F, t25523: F, t26128: F, t28: F, t32011: F, t32338: F, t3266: F, t34354: F, t34565: F, t34568: F, t492: F, t5495: F, t5501: F, t6421: F, t8411: F, t8418: F) -> (F, F, F) {
    let t145705 = t488 * (t144826 + t144888 + t144943 + t144981 + t145034 + t145603 + t145665 + t145701);
    let t145719 = t22943 * t25598;
    let t145731 = t6414 * t31995;
    let t145733 = t137476 / 9.0 - 2.0 * t144765 - 2.0 * t145705 + t1286 * t28 * t32338 * t25523 + t1286 * t28 * t32338 * t26128 - 24.0 * t8418 * t34565 * t492 - 12.0 * t8418 * t34568 * t492 + 8.0 * t145719 - t137497 / 18.0 + t5501 * t8411 * t32011 * t3266 - t1286 * t28 * t136116 * t6421 / 3.0 - 2.0 / 3.0 * t5495 * t34354 - t145731 / 9.0;
    (t145705, t145719, t145733)
}
