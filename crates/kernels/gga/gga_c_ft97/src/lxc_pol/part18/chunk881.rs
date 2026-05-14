//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 881/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk881<F: Float>(t2190: F, t23571: F, t12968: F, t5857: F, t8392: F, t5856: F, t9419: F, t160: F, t5842: F, t379: F, t2221: F, t574: F, t5935: F, t1391: F, t1651: F, t569: F) -> (F, F, F, F, F, F, F, F, F) {
    let t23572 = t23571 * t2190;
    let t23573 = t12968 * t23572;
    let t23576 = t8392 * t5857;
    let t23578 = t9419 * t5856;
    let t23581 = t160 * t5842;
    let t23582 = t23581 * t379;
    let t23583 = t2221 * t23582;
    let t23587 = t574 * t5935 * t2190;
    let t23591 = t569 * t1391 * t1651;
    (t23572, t23573, t23576, t23578, t23581, t23582, t23583, t23587, t23591)
}
