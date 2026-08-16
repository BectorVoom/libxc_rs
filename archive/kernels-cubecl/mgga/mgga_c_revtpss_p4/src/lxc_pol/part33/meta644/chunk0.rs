//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2093/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2093<F: Float>(t26865: F, t370: F, t17727: F, t17423: F, t29097: F, t17789: F, t29100: F, t17416: F, t7624: F, t17608: F, t7617: F, t17217: F, t26880: F) -> (F, F, F, F, F, F, F) {
    let t104646 = t26865 * t370;
    let t104647 = t17727 * t104646;
    let t104651 = F::cast_from(0.11433071498151929859e-2_f64) * t29097 * t17423;
    let t104653 = F::cast_from(0.57165357490759649296e-3_f64) * t29100 * t17789;
    let t104658 = t7624 * t17416;
    let t104677 = t17608 * t7617;
    let t104680 = t26880 * t17217;
    (t104646, t104647, t104651, t104653, t104658, t104677, t104680)
}
