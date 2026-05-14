//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 983/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk983<F: Float>(t1882: F, t6471: F, t379: F, t6478: F, t8557: F, t11863: F, t25933: F, t25919: F, t1901: F, t23176: F, t26276: F, t26280: F, t26284: F, t26288: F, t26291: F, t26293: F, t26295: F, t26297: F, t26301: F, t446: F) -> (F, F, F, F, F) {
    let t26303 = t1882 * t6471;
    let t26305 = t6478 * t379;
    let t26306 = t8557 * t26305;
    let t26309 = t11863 * t25933;
    let t26312 = t11863 * t25919;
    let t26315 = -t446 * t26276 / 3.0 - t446 * t26280 / 3.0 - t446 * t26284 / 3.0 - t446 * t26288 / 3.0 + t26291 / 9.0 + t26293 / 9.0 + t26295 / 9.0 + 2.0 / 27.0 * t1901 * t26297 - t23176 / 9.0 - t26301 / 9.0 - 2.0 / 9.0 * t26303 - t1901 * t26306 / 9.0 - 2.0 / 9.0 * t1901 * t26309 - 2.0 / 9.0 * t1901 * t26312;
    (t26305, t26306, t26309, t26312, t26315)
}
