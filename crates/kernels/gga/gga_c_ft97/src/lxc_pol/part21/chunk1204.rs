//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1204/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1204<F: Float>(t102524: F, t102689: F, t103472: F, t103827: F, t11490: F, t115289: F, t11593: F, t11902: F, t15951: F, t15955: F, t15959: F, t15994: F, t16047: F, t16170: F, t16198: F, t16305: F, t1901: F, t23249: F, t23323: F, t26188: F, t26372: F, t26373: F, t26378: F, t26382: F, t29822: F, t29914: F, t379: F, t446: F, t4495: F, t452: F, t488: F, t5743: F, t59631: F, t60901: F, t83: F, t8372: F, t8557: F, t91771: F) -> (F,) {
    let t117649 = -t1901 * t8557 * t29822 * t379 / 9.0 - 4.0 / 9.0 * t1901 * t102689 * t15959 - 2.0 / 9.0 * t1901 * t91771 * t16305 - 4.0 / 9.0 * t1901 * t102524 * t15951 + 4.0 / 27.0 * t1901 * t103472 * t15955 + 4.0 / 3.0 * t446 * t83 * t115289 + t446 * t452 * t488 * t5743 * t4495 / 3.0 + 2.0 * t1901 * t11490 * t26373 * t15994 + 8.0 * t1901 * t26372 * t103827 * t16198 - 4.0 / 3.0 * t1901 * t60901 * t26378 - 4.0 / 3.0 * t1901 * t59631 * t26382 + t1901 * t8372 * t29914 / 9.0 + 8.0 / 9.0 * t11593 * t23323 * t16170 - 4.0 / 3.0 * t1901 * t11490 * t23249 * t16047 + 4.0 / 9.0 * t11593 * t11902 * t26188;
    (t117649,)
}
