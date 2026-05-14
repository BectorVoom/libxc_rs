//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1303/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1303<F: Float>(t1882: F, t30321: F, t30385: F, t30424: F, t8392: F, t30394: F, t119445: F, t119479: F, t12714: F, t144: F, t1557: F, t16956: F, t17017: F, t17409: F, t1901: F, t23463: F, t23470: F, t26849: F, t30105: F, t3052: F, t3188: F, t3281: F, t3439: F, t3578: F, t4454: F, t446: F, t4828: F, t569: F, t574: F, t5947: F, t616: F, t6718: F, t6725: F, t95767: F) -> (F,) {
    let t120501 = t1882 * t30321;
    let t120523 = t1882 * t30385;
    let t120531 = t8392 * t30424;
    let t120547 = t8392 * t30394;
    let t120549 = t120501 / 9.0 + 2.0 / 27.0 * t1901 * t3439 * t23463 * t4454 - 4.0 / 9.0 * t3281 * t569 * t6725 * t3052 + 2.0 / 3.0 * t446 * t144 * t119479 + 4.0 / 27.0 * t1901 * t12714 * t6718 * t1557 * t3188 + t446 * t574 * t17409 * t5947 / 3.0 - t120523 / 9.0 - 2.0 / 9.0 * t1901 * t23470 * t16956 - t446 * t144 * t119445 / 3.0 - 2.0 / 27.0 * t120531 - t446 * t574 * t616 * t30105 / 3.0 + 2.0 / 9.0 * t1901 * t95767 * t4828 + 2.0 / 3.0 * t446 * t574 * t3578 * t26849 - 2.0 / 9.0 * t1901 * t23470 * t17017 + 4.0 / 9.0 * t120547;
    (t120549,)
}
