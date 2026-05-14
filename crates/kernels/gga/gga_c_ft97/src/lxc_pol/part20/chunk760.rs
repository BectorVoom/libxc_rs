//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 760/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk760<F: Float>(t24395: F, t675: F, t263: F, t193: F, t2469: F, t6187: F, t2526: F, t6154: F, t1449: F, t2569: F, t10052: F, t10153: F, t1443: F, t2567: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24396 = t675 * t24395;
    let t24397 = t24396 * t263;
    let t24398 = t193 * t24397;
    let t24403 = t2469 * t6187;
    let t24405 = t6154 * t2526;
    let t24407 = t1449 * t2569;
    let t24408 = t10052 * t24407;
    let t24410 = t10153 * t1449;
    let t24412 = t1443 * t2567;
    (t24396, t24397, t24398, t24403, t24405, t24407, t24408, t24410, t24412)
}
