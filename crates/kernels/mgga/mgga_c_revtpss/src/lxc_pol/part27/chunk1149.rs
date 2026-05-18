//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1149/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1149<F: Float>(t25956: F, t26087: F, t532: F, t1450: F, t2014: F, t2042: F, t4158: F, t1459: F, t7331: F, t7334: F, t1936: F, t2327: F) -> (F, F, F, F, F, F, F, F) {
    let t26088 = t25956 + t26087;
    let t26089 = t532 * t26088;
    let t26090 = t26089 * t1450;
    let t26091 = t2014 * t26090;
    let t26115 = F::new(3.0) * t4158 * t2042;
    let t26117 = F::new(12.0) * t1459 * t7331;
    let t26119 = F::new(6.0) * t1459 * t7334;
    let t26120 = t2327 * t1936;
    (t26088, t26089, t26090, t26091, t26115, t26117, t26119, t26120)
}
