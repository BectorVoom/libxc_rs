//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1173/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1173<F: Float>(t32195: F, t32206: F, t3936: F, t5591: F, t121204: F, t1868: F, t9818: F, t121232: F, t1353: F, t1903: F, t120956: F, t1414: F, t828: F) -> (F, F, F, F, F) {
    let t125659 = t32206 * t3936 * t32195 * t5591;
    let t125662 = t9818 * t121204 * t1868;
    let t125663 = t121232 * t125662;
    let t125668 = t1903 * t1353;
    let t125671 = t120956 * t1414 * t828 * t125668;
    (t125659, t125662, t125663, t125668, t125671)
}
