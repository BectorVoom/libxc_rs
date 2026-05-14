//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 993/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk993<F: Float>(t69117: F, t69153: F, t1240: F, t2681: F, t4125: F, t820: F, t1208: F, t2735: F, t2726: F, t51: F, t6247: F, t2691: F, t28666: F, t1200: F, t7606: F, t19106: F, t800: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t69154 = t69117 + t69153;
    let t69996 = t2681 * t1240;
    let t70435 = t4125 * t820;
    let t70440 = t1208 * t2735;
    let t70452 = t1208 * t2726;
    let t70456 = t6247 * t51;
    let t70457 = t2691 * t70456;
    let t70458 = t28666 * t820;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    (t69154, t69996, t70435, t70440, t70452, t70456, t70457, t70458, t70497, t70550)
}
