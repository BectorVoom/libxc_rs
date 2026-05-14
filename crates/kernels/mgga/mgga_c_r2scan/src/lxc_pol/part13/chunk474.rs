//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 474/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk474<F: Float>(t2185: F, t552: F, t551: F, t1632: F, t560: F, t549: F, t146: F, t1541: F) -> (F, F, F, F, F, F) {
    let t2186 = t552 * t2185;
    let t2187 = t551 * t2186;
    let t2190 = t1632 * t560;
    let t2191 = t551 * t2190;
    let t2192 = t549 * t2191;
    let t2195 = t146 * t1541;
    (t2186, t2187, t2190, t2191, t2192, t2195)
}
