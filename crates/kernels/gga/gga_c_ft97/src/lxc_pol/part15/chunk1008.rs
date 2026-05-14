//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1008/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1008<F: Float>(t66832: F, t80677: F, t80679: F, t88143: F, t88147: F, t88151: F, t88155: F, t88159: F, t88163: F, t88167: F, t88171: F, t88178: F, t88182: F, t88186: F, t88190: F, t52212: F, t52916: F, t66902: F, t66905: F, t66934: F, t66945: F, t67420: F, t80685: F, t80696: F, t80759: F, t80770: F, t80772: F, t88198: F, t88201: F, t88213: F) -> (F, F) {
    let t89497 = 40.0 / 81.0 * t88143 + 4.0 / 9.0 * t88147 + 8.0 / 9.0 * t88151 - 8.0 / 27.0 * t88155 - 4.0 / 3.0 * t88159 - 16.0 / 9.0 * t88163 + 8.0 / 3.0 * t88167 + 8.0 / 3.0 * t88171 + 16.0 / 27.0 * t66832 - 8.0 / 9.0 * t80677 + 8.0 / 9.0 * t80679 + 8.0 / 3.0 * t88178 + 2.0 / 3.0 * t88182 + 8.0 / 3.0 * t88186 + 4.0 / 9.0 * t88190;
    let t89513 = 8.0 / 3.0 * t80685 - 8.0 / 9.0 * t66902 + 16.0 / 9.0 * t66905 + 8.0 / 9.0 * t88198 - 8.0 / 3.0 * t88201 + 8.0 / 9.0 * t80696 + 16.0 / 27.0 * t66934 - 8.0 / 27.0 * t66945 + 112.0 / 243.0 * t52212 + 112.0 / 81.0 * t52916 - 16.0 / 27.0 * t80759 - 16.0 / 81.0 * t67420 + 8.0 / 27.0 * t80770 - 8.0 / 27.0 * t80772 + 3.0 / 4.0 * t88213;
    (t89497, t89513)
}
