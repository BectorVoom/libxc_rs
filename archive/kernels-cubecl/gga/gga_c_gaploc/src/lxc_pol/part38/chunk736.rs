//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 736/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk736<F: Float>(t1406: F, t6715: F, t1564: F, t588: F, t16879: F, t486: F, t165: F, t2089: F, t16534: F, t169: F, t10913: F, t2021: F) -> (F, F, F, F, F, F) {
    let t21370 = t1406 * t6715;
    let t21373 = t588 * t1564;
    let t21501 = t16879 * t486;
    let t21502 = t165 * t2089;
    let t22090 = t16534 * t169;
    let t22242 = t2021 * t10913;
    (t21370, t21373, t21501, t21502, t22090, t22242)
}
