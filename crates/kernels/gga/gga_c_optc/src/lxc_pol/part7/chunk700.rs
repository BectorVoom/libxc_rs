//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 700/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk700<F: Float>(t6646: F, t1956: F, t732: F, t103: F, t193: F, t197: F, t652: F, t102: F, t133: F, t751: F, t1928: F, t745: F) -> (F, F, F, F, F, F) {
    let t6647 = F::cast_from(0.51947267698127589899e2_f64) * t6646;
    let t6648 = t732 * t1956;
    let t6653 = F::new(15400.0) / F::new(243.0) * t193 * t652 * t103 * t197;
    let t6654 = t133 * t102;
    let t6656 = t193 * t6654 * t751;
    let t6660 = t193 * t745 * t1928 * t197;
    (t6647, t6648, t6653, t6654, t6656, t6660)
}
