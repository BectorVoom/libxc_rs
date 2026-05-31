//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 895/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk895<F: Float>(t42513: F, t12844: F, t501: F, t605: F, t2358: F, t33959: F, t27214: F, t9253: F, t10624: F, t1382: F, t921: F, t1365: F, t31558: F, t6525: F) -> (F, F, F, F, F, F) {
    let t42514 = F::cast_from(2.0_f64) * t42513;
    let t42515 = t12844 * t501;
    let t42516 = t42515 * t605;
    let t42517 = t33959 * t2358;
    let t42518 = F::cast_from(4.0_f64) * t42517;
    let t42520 = F::cast_from(6.0_f64) * t27214 * t9253;
    let t42522 = t1382 * t10624 * t921;
    let t42523 = F::cast_from(4.0_f64) * t42522;
    let t42529 = t6525 * t1365 * t31558;
    (t42514, t42516, t42518, t42520, t42523, t42529)
}
