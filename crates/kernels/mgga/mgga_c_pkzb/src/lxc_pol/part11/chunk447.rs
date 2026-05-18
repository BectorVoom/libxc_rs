//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 447/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk447<F: Float>(t197: F, t2036: F, t2025: F, t2029: F, t287: F, t154: F, t277: F, t486: F, t276: F, t301: F, t67: F) -> (F, F, F, F, F, F) {
    let t2037 = t2036 * t197;
    let t2038 = t2037 * t2025;
    let t2039 = t2029 * t287;
    let t2045 = t154 * t486 * t277;
    let t2047 = t276 * t2045 / F::new(432.0);
    let t2048 = t67 * t301;
    (t2037, t2038, t2039, t2045, t2047, t2048)
}
