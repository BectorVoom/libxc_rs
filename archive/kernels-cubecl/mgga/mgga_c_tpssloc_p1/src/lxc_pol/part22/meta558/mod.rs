//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta558 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta558<F: Float>(t41362: F, t831: F, t2628: F, t2690: F, t812: F, t835: F, t9972: F, t2617: F, t9666: F, t776: F, t9975: F, t6589: F, t67: F, t246: F, t22715: F, t268: F, t271: F, t10969: F, t154: F, t2769: F, t885: F, t9698: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t41363, t41385, t41414, t41424, t41453, t41466) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2061::<F>(t41362, t831, t2628, t2690, t812, t835, t9972, t2617, t9666, t776, t9975, t6589, t67);
        let (t41467, t41654, t41655, t41664, t41666, t41684) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2062::<F>(t246, t41466, t22715, t268, t271, t10969, t154, t2769, t885, t9698);
    (t41363, t41385, t41414, t41424, t41453, t41466, t41467, t41654, t41655, t41664, t41666, t41684)
}
