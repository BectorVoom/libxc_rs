//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1111/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1111<F: Float>(t125648: F, t1381: F, t8590: F, t121181: F, t5741: F, t121146: F, t32195: F, t32206: F, t3936: F, t5591: F, t121204: F, t1868: F, t9818: F) -> (F, F, F, F, F) {
    let t125650 = t125648 * t8590 * t1381;
    let t125652 = t121181 * t5741;
    let t125654 = t121146 * t5741;
    let t125659 = t32206 * t3936 * t32195 * t5591;
    let t125662 = t9818 * t121204 * t1868;
    (t125650, t125652, t125654, t125659, t125662)
}
