//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2321/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2321<F: Float>(t40741: F, t40743: F, t40748: F, t40760: F, t40764: F, t40766: F, t46292: F, t67162: F, t67163: F, t67169: F, t67170: F, t67174: F, t67176: F, t67178: F, t67180: F, t67183: F, t67186: F) -> F {
    let t67452 = -t67162 + t67163 + t67169 - t67170 - t40741 - t40743 + t40748 + t67174 + t40760 - t67176 + t46292 + t67178 + t40764 + t40766 + t67180 + t67183 + t67186;
    t67452
}
