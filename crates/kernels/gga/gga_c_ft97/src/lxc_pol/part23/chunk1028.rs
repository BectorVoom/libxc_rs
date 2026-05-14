//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1028/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1028<F: Float>(t10007: F, t31302: F, t24569: F, t5170: F, t14175: F, t242: F, t30954: F, t30950: F, t1901: F, t24815: F, t28382: F, t28384: F, t28408: F, t28411: F, t28451: F, t28453: F, t31273: F, t31279: F, t31283: F, t31286: F, t31289: F, t31293: F, t31298: F, t446: F) -> (F, F, F, F, F, F) {
    let t31303 = t10007 * t31302;
    let t31306 = t24569 * t5170;
    let t31307 = t14175 * t31306;
    let t31312 = t242 * t30954;
    let t31315 = t242 * t30950;
    let t31318 = 2.0 / 9.0 * t446 * t31273 - 2.0 / 9.0 * t28382 - 4.0 / 9.0 * t28384 - t446 * t31279 / 3.0 - 2.0 / 3.0 * t446 * t31283 - 2.0 / 3.0 * t446 * t31286 + 2.0 / 3.0 * t446 * t31289 - t446 * t31293 / 3.0 - t24815 - 2.0 / 27.0 * t28408 + 2.0 / 3.0 * t446 * t31298 - 2.0 / 9.0 * t28411 - 2.0 / 9.0 * t1901 * t31303 - 4.0 / 9.0 * t1901 * t31307 + 2.0 / 9.0 * t28451 + 2.0 / 9.0 * t28453 - 2.0 / 3.0 * t446 * t31312 - t446 * t31315 / 3.0;
    (t31303, t31306, t31307, t31312, t31315, t31318)
}
