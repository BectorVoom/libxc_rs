//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 832/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk832<F: Float>(t1382: F, t2497: F, t3418: F, t32100: F, t921: F, t2358: F, t33959: F, t27214: F, t9253: F, t10624: F, t1365: F, t31558: F, t6525: F, t12963: F, t1358: F, t2299: F, t488: F) -> (F, F, F, F, F, F, F) {
    let t42511 = t1382 * t3418 * t2497;
    let t42513 = t32100 * t921;
    let t42517 = t33959 * t2358;
    let t42520 = 6.0 * t27214 * t9253;
    let t42522 = t1382 * t10624 * t921;
    let t42529 = t6525 * t1365 * t31558;
    let t42533 = t1358 * t2299 * t12963 * t488;
    (t42511, t42513, t42517, t42520, t42522, t42529, t42533)
}
