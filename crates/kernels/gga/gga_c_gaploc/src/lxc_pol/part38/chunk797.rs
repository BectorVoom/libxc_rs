//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 797/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk797<F: Float>(t34604: F, t544: F, t9287: F, t12938: F, t2464: F, t587: F, t26435: F, t6710: F, t9438: F, t12990: F, t30733: F, t10122: F, t2465: F) -> (F, F, F, F, F) {
    let t42369 = t544 * t34604 * t9287;
    let t42378 = t587 * t2464 * t12938;
    let t42400 = t6710 * t9438 * t26435;
    let t42412 = t12990 * t30733;
    let t42416 = t587 * t2464 * t2465 * t10122;
    (t42369, t42378, t42400, t42412, t42416)
}
