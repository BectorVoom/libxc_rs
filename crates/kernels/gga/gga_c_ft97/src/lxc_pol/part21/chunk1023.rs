//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1023/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1023<F: Float>(t1742: F, t420: F, t409: F, t5532: F, t122: F, t1301: F, t2248: F, t71: F, t1300: F, t1608: F, t22532: F, t5596: F, t22759: F, t5567: F, t5578: F, t625: F) -> (F, F, F, F, F, F, F, F) {
    let t92482 = t420 * t1742;
    let t92488 = t409 * t5532;
    let t92515 = t5532 * t122;
    let t92529 = t1301 * t2248 * t71;
    let t92531 = 0.70937342644032921812e-2 * t1300 * t92529;
    let t92533 = t1608 * t5596 * t22532;
    let t92538 = t1608 * t22759 * t5567;
    let t92557 = t5578 * t625;
    (t92482, t92488, t92515, t92529, t92531, t92533, t92538, t92557)
}
