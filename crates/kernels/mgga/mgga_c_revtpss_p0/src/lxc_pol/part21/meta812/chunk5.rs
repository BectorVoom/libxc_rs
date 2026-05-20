//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2974/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2974<F: Float>(t51840: F, t51844: F, t51846: F, t52141: F, t52146: F, t52150: F, t52153: F, t52156: F, t52159: F, t52162: F, t52166: F, t52170: F, t52174: F, t52176: F, t52178: F, t52180: F, t52182: F, t52185: F, t52187: F, t52194: F) -> F {
    let t54230 = t51840 - t51844 + t51846 - t52141 - t52146 + t52150 - t52153 - t52156 - t52159 + t52162 + t52166 + t52170 + t52174 - t52176 - t52178 + t52180 + t52182 - t52185 - t52187 + t52194;
    t54230
}
