//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1176/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1176<F: Float>(t7108: F, t993: F, t21389: F, t7058: F, t974: F, t2536: F, t2558: F, t365: F, t21424: F, t2561: F, t2530: F, t2537: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21616 = t993 * t7108;
    let t21620 = F::cast_from(0.17757530864197530864e0_f64) * t21389;
    let t21628 = t974 * t7058;
    let t21633 = t365 / t2558 / t2536;
    let t21638 = F::cast_from(0.5356037037037037037e1_f64) * t21389;
    let t21641 = F::cast_from(0.16979925925925925926e1_f64) * t21424;
    let t21668 = t2558 * t2558;
    let t21670 = t365 / t21668;
    let t21671 = t2561 * t2561;
    let t21672 = F::new(1.0) / t21671;
    let t21676 = t2530 * t2537;
    (t21616, t21620, t21628, t21633, t21638, t21641, t21670, t21672, t21676)
}
