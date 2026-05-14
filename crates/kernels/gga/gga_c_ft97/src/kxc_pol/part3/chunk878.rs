//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 878/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk878<F: Float>(t15299: F, t18997: F, t5424: F, t684: F, t835: F, t4176: F, t4246: F, t840: F, t10732: F, t10735: F, t15318: F, t15329: F, t15334: F, t15336: F, t15376: F, t15382: F, t15384: F, t15400: F, t15419: F, t15420: F, t1901: F, t446: F) -> (F,) {
    let t19793 = t15299 * t18997;
    let t19799 = t835 * t5424 * t684;
    let t19803 = t840 * t4246 * t4176;
    let t19809 = -4.0 / 9.0 * t1901 * t19793 - 8.0 / 81.0 * t15318 + 8.0 / 27.0 * t15329 + t15334 + t15336 - t15376 - t15382 - t15384 + t15400 - t446 * t19799 / 9.0 + 2.0 / 3.0 * t446 * t19803 - 4.0 / 81.0 * t10732 + t15419 - 8.0 / 27.0 * t15420 - 4.0 / 27.0 * t10735;
    (t19809,)
}
