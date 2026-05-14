//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1015/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1015<F: Float>(t89656: F, t89684: F, t66833: F, t80677: F, t80679: F, t88143: F, t88147: F, t88151: F, t88155: F, t88159: F, t88163: F, t88167: F, t88171: F, t88178: F, t88182: F, t66903: F, t66906: F, t66935: F, t66946: F, t67421: F, t68751: F, t68774: F, t80685: F, t80696: F, t80759: F, t88186: F, t88190: F, t88198: F, t88201: F) -> (F, F, F) {
    let t89685 = t89656 + t89684;
    let t89704 = 20.0 / 81.0 * t88143 + 2.0 / 9.0 * t88147 + 4.0 / 9.0 * t88151 - 4.0 / 27.0 * t88155 - 2.0 / 3.0 * t88159 - 8.0 / 9.0 * t88163 + 4.0 / 3.0 * t88167 + 4.0 / 3.0 * t88171 + t66833 - 4.0 / 9.0 * t80677 + 4.0 / 9.0 * t80679 + 4.0 / 3.0 * t88178 + t88182 / 3.0;
    let t89712 = 4.0 / 3.0 * t88186 + 2.0 / 9.0 * t88190 + 4.0 / 3.0 * t80685 - t66903 + t66906 + 4.0 / 9.0 * t88198 - 4.0 / 3.0 * t88201 + 4.0 / 9.0 * t80696 + t66935 - t66946 + t68751 + t68774 - 8.0 / 27.0 * t80759 - t67421;
    (t89685, t89704, t89712)
}
