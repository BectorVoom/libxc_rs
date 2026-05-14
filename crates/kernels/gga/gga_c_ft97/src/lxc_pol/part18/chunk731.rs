//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 731/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk731<F: Float>(t12561: F, t526: F, t27: F, t89: F, t358: F, t582: F, t2999: F, t1018: F, t1636: F, t10998: F, t569: F, t446: F, t11003: F, t3281: F, t11034: F, t2205: F) -> (F, F, F, F, F, F) {
    let t12562 = t526 * t12561;
    let t12564 = t89 * t27 * t12562;
    let t12566 = t582 * t358;
    let t12568 = t89 * t2999 * t12566;
    let t12571 = t89 * t1636 * t1018;
    let t12573 = t569 * t10998;
    let t12574 = t446 * t12573;
    let t12576 = t569 * t11003;
    let t12577 = t3281 * t12576;
    let t12579 = t2205 * t11034;
    (t12564, t12568, t12571, t12574, t12577, t12579)
}
