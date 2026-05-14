//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1223/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1223<F: Float>(t22914: F, t29586: F, t29740: F, t92: F, t103972: F, t103975: F, t117998: F, t118141: F, t1337: F, t15596: F, t16011: F, t16241: F, t22907: F, t22908: F, t22917: F, t22922: F, t25609: F, t25610: F, t29461: F, t29741: F, t3109: F, t4454: F, t5495: F, t5501: F, t5504: F, t5510: F, t6562: F, t7793: F) -> (F,) {
    let t118406 = t22914 * t29586;
    let t118408 = t29740 * t92;
    let t118434 = t5495 * t29461 / 6.0 + t103972 - t15596 * t1337 + t118406 / 54.0 - t118408 * t5504 / 18.0 + 4.0 / 27.0 * t103975 - 4.0 * t118141 - t29741 * t5510 / 3.0 + t5501 * t22907 * t22908 * t16241 / 9.0 + t5501 * t25609 * t25610 * t16011 / 9.0 - 4.0 * t117998 - 2.0 * t3109 * t6562 - t5501 * t7793 * t22917 * t4454 / 27.0 - t5501 * t7793 * t22922 * t4454 / 27.0;
    (t118434,)
}
