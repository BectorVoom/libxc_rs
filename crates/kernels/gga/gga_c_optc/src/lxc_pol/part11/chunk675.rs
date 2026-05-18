//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 675/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk675<F: Float>(t1867: F, t6407: F, t3649: F, t3696: F, t6364: F, t6367: F, t6370: F, t6375: F, t6377: F, t6379: F, t587: F, t1863: F, t579: F) -> (F, F, F, F) {
    let t6408 = t6407 * t1867;
    let t6419 = -F::new(0.34523333333333333333e1) * t6364 + F::new(0.23015555555555555556e1) * t6367 - F::new(0.26851481481481481482e1) * t6370 - F::new(0.93932222222222222223e0) * t3649 + F::new(0.73355e-1) * t6375 - F::new(0.14671e0) * t6377 - F::new(0.17116166666666666667e0) * t6379 - F::new(0.36793333333333333333e0) * t3696;
    let t6420 = t6419 * t587;
    let t6424 = F::new(1.0) / t1863 / t579;
    (t6408, t6419, t6420, t6424)
}
