//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 453/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk453<F: Float>(t2620: F, t331: F, t287: F, t530: F, t321: F, t320: F, t327: F, t301: F, t2665: F, t305: F, t140: F, t2661: F) -> (F, F, F, F, F, F, F, F) {
    let t2729 = F::cast_from(0.16793568152788065763e-2_f64) * t331 * t2620;
    let t2742 = t530 * t287;
    let t2743 = t321 * t2742;
    let t2745 = F::cast_from(0.19318136643975017455e-1_f64) * t320 * t2743;
    let t2746 = t327 * t327;
    let t2747 = F::new(1.0) / t2746;
    let t2748 = t2747 * t301;
    let t2749 = t305 * t2665;
    let t2750 = t2749 * t140;
    let t2751 = t2748 * t2750;
    let t2758 = t2661 * t2750;
    (t2729, t2742, t2745, t2746, t2747, t2748, t2751, t2758)
}
