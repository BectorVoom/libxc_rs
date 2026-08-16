//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2081/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2081<F: Float>(t6897: F, t6907: F, t90544: F, t26203: F, t6883: F, t7700: F, t80645: F, t225: F, t26219: F, t214: F, t5318: F, t26378: F, t6914: F) -> (F, F, F, F, F, F) {
    let t90701 = t6897 * t90544 * t6907;
    let t90702 = F::cast_from(0.82246703342411321824e-2_f64) * t90701;
    let t90707 = t6883 * t26203;
    let t90708 = F::cast_from(0.38381794893125283518e-1_f64) * t90707;
    let t90723 = t6897 * t80645 * t7700;
    let t90724 = F::cast_from(0.82246703342411321824e-2_f64) * t90723;
    let t90732 = t26219 * t225;
    let t90739 = t214 * t5318;
    let t90749 = t6914 * t26378;
    (t90702, t90708, t90724, t90732, t90739, t90749)
}
