//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1392/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1392<F: Float>(t11166: F, t948: F, t969: F, t10876: F, t2516: F, t2520: F, t4238: F, t2524: F, t1410: F, t25273: F, t3514: F, t9099: F) -> (F, F, F, F, F) {
    let t30196 = t11166 * t948;
    let t30198 = F::new(2.0) * t30196 * t969;
    let t30200 = F::new(1.0) * t10876 * t2516;
    let t30201 = t4238 * t2520;
    let t30203 = F::cast_from(0.16081979498692535067e2_f64) * t30201 * t2524;
    let t30205 = F::new(2.0) * t25273 * t1410;
    let t30207 = F::new(4.0) * t9099 * t3514;
    (t30198, t30200, t30203, t30205, t30207)
}
