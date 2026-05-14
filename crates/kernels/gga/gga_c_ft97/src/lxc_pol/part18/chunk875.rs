//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 875/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk875<F: Float>(t23478: F, t609: F, t144: F, t1901: F, t23417: F, t23422: F, t23425: F, t23427: F, t23431: F, t23436: F, t23440: F, t23444: F, t23448: F, t23452: F, t23457: F, t23460: F, t23465: F, t23468: F, t23471: F, t23475: F, t446: F) -> (F, F, F) {
    let t23479 = t23478 * t609;
    let t23480 = t144 * t23479;
    let t23483 = -2.0 / 9.0 * t1901 * t23417 + 2.0 / 3.0 * t446 * t23422 - 2.0 / 9.0 * t23425 - 4.0 / 9.0 * t23427 + 2.0 / 3.0 * t446 * t23431 + t446 * t23436 / 3.0 + 2.0 / 3.0 * t446 * t23440 + 2.0 / 9.0 * t1901 * t23444 + t1901 * t23448 / 9.0 + 2.0 / 27.0 * t1901 * t23452 - 4.0 / 3.0 * t1901 * t23457 + 2.0 / 9.0 * t1901 * t23460 + 2.0 / 9.0 * t1901 * t23465 - 2.0 / 27.0 * t23468 + 2.0 / 9.0 * t1901 * t23471 - t446 * t23475 / 3.0 - 2.0 / 3.0 * t446 * t23480;
    (t23479, t23480, t23483)
}
