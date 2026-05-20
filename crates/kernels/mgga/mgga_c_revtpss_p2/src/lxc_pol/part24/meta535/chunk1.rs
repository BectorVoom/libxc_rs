//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1576/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1576<F: Float>(t22865: F, t9918: F, t1883: F, t6883: F, t9816: F, t9818: F, t13999: F, t22833: F, t22813: F, t547: F, t807: F, t9941: F) -> (F, F, F, F) {
    let t86112 = t9918 * t22865;
    let t86124 = t9816 * t9818 * t6883 * t1883;
    let t86156 = t13999 * t22833;
    let t86165 = t807 * t547 * t9941 * t22813;
    (t86112, t86124, t86156, t86165)
}
