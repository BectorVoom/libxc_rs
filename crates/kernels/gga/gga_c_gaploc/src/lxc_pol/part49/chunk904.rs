//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 904/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk904<F: Float>(t12996: F, t18067: F, t2365: F, t31586: F, t4391: F, t31591: F, t12993: F, t7014: F, t10215: F, t123: F, t883: F, t2487: F, t2488: F) -> (F, F, F, F, F, F) {
    let t41623 = t18067 * t12996;
    let t41624 = F::new(0.59584149919750711116e-1) * t41623;
    let t41626 = t4391 * t2365 * t31586;
    let t41627 = F::new(0.59584149919750711116e-1) * t41626;
    let t41629 = t4391 * t2365 * t31591;
    let t41630 = F::new(0.59584149919750711116e-1) * t41629;
    let t41631 = t7014 * t12993;
    let t41634 = t10215 * t123 * t883;
    let t41636 = t2487 * t2488 * t41634;
    (t41624, t41627, t41630, t41631, t41634, t41636)
}
