//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2063/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2063<F: Float>(t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F, t26211: F, t6883: F, t268: F, t557: F, t6559: F, t26333: F, t81326: F) -> (F, F, F, F, F, F) {
    let t90582 = t22724 * t26474;
    let t90584 = t22751 * t26194;
    let t90585 = F::cast_from(0.76763589786250567036e-1_f64) * t90584;
    let t90591 = t80830 * t1887;
    let t90604 = t6883 * t26211;
    let t90605 = F::cast_from(0.38381794893125283518e-1_f64) * t90604;
    let t90607 = t6559 * t557 * t268;
    let t90609 = t90607 * t81326 * t26333;
    (t90582, t90585, t90591, t90605, t90607, t90609)
}
