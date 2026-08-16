//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1932/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1932<F: Float>(t26301: F, t80853: F, t80855: F, t22788: F, t5314: F, t16333: F, t6952: F, t1831: F, t80866: F, t131: F, t6931: F, t9537: F) -> (F, F, F, F, F) {
    let t91143 = t80853 * t80855 * t26301;
    let t91145 = t22788 * t5314;
    let t91147 = t6952 * t16333;
    let t91149 = t80866 * t1831;
    let t91152 = t6931 * t131 * t9537;
    (t91143, t91145, t91147, t91149, t91152)
}
