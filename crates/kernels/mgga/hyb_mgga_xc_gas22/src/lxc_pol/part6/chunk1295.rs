//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1295/1345 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1295<F: Float>(t1014: F, t10865: F, t10868: F, t11173: F, t11174: F, t2579: F, t2609: F, t30127: F, t30182: F, t30184: F, t30198: F, t30200: F, t30203: F, t3591: F, t7165: F, t9282: F, t9289: F) -> (F,) {
    let t30348 = t30127 + t30182 + t30184 + 0.10389515463408878255e3 * t1014 * t11173 * t7165 + 0.14035736694323150897e2 * t1014 * t11173 * t2579 - 0.69263436422725855034e2 * t3591 * t9289 - 0.34631718211362927518e2 * t2609 * t10865 + 0.23392894490538584828e1 * t3591 * t9282 - 0.69263436422725855036e2 * t2609 * t10868 + 0.2077903092681775651e3 * t2609 * t11174 + t30198 + t30200 + t30203;
    (t30348,)
}
