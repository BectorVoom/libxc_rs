//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2797/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2797<F: Float>(t14519: F, t2470: F, t2798: F, t231: F, t51049: F, t2782: F, t2797: F, t14663: F, t686: F, t72: F, t4522: F, t874: F, t9288: F) -> (F, F, F, F) {
    let t51434 = t2798 * t14519 * t2470;
    let t51435 = F::cast_from(0.39029762157531132076e-1_f64) * t51434;
    let t51436 = t51049 * t231;
    let t51438 = t2782 * t2797 * t51436;
    let t51442 = t2798 * t14663 * t72 * t686;
    let t51445 = t874 * t4522 * t9288;
    (t51435, t51438, t51442, t51445)
}
