//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1321/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1321<F: Float>(t3596: F, t7627: F, t26936: F, t3566: F, t13181: F, t3140: F, t1243: F, t2149: F, t2147: F, t44841: F, t7635: F, t3572: F, t8945: F) -> (F, F, F, F, F) {
    let t97332 = t3596 * t7627;
    let t97343 = t3566 * t26936;
    let t97346 = t3140 * t13181;
    let t97348 = t2149 * t97346 * t1243;
    let t97358 = t2147 * t44841 * t7635;
    let t97363 = t3572 * t8945;
    (t97332, t97343, t97348, t97358, t97363)
}
