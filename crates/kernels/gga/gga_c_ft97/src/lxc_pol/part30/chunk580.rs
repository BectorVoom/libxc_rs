//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 580/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk580<F: Float>(t28023: F, t766: F, t24232: F, t3875: F, t24231: F, t1425: F, t683: F, t2360: F, t263: F, t3886: F, t2404: F, t2347: F, t6752: F, t684: F, t24455: F, t24470: F, t27466: F, t27471: F, t27473: F, t27477: F, t27481: F, t27485: F, t27745: F, t27751: F, t27755: F, t27759: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28024 = t28023 * t766;
    let t28026 = t24232 * t3875;
    let t28027 = t24231 * t28026;
    let t28030 = t683 * t1425;
    let t28031 = t263 * t2360;
    let t28032 = t28031 * t3886;
    let t28033 = t28030 * t28032;
    let t28036 = t2404 * t1425;
    let t28037 = t263 * t2347;
    let t28038 = t28037 * t3886;
    let t28039 = t28036 * t28038;
    let t28042 = t6752 * t684;
    let t28043 = t24231 * t28042;
    let t28057 = t27466 / 18.0 + t27471 / 9.0 - t27473 / 27.0 - 2.0 / 9.0 * t27477 - 2.0 * t27481 + t27485 / 9.0 - t27745 / 6.0 - t24455 / 36.0 - t24470 / 9.0 - t27751 - t27755 / 9.0 - t27759 / 9.0;
    (t28024, t28026, t28027, t28030, t28032, t28033, t28036, t28038, t28039, t28042, t28043, t28057)
}
