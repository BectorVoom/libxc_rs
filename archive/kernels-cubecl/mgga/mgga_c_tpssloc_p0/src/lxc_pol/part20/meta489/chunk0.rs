//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1981/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1981<F: Float>(t1307: F, t16195: F, t3719: F, t5279: F, t1347: F, t16018: F, t1345: F, t1348: F, t16176: F, t16186: F, t16192: F, t1819: F, t1821: F, t3839: F, t3844: F, t3847: F, t5272: F, t5278: F, t5280: F, t5283: F, t546: F, t548: F) -> (F, F, F, F) {
    let t16196 = t16195 * t1307;
    let t16199 = t5279 * t3719;
    let t16202 = t1347 * t16018;
    let t16205 = F::cast_from(6.0_f64) * t1345 * t5283 + F::cast_from(6.0_f64) * t1348 * t5272 - t16176 * t548 - F::cast_from(24.0_f64) * t16186 * t5280 + F::cast_from(60.0_f64) * t16192 * t5278 - F::cast_from(24.0_f64) * t16196 * t5278 - F::cast_from(12.0_f64) * t16199 * t5278 + F::cast_from(3.0_f64) * t16202 * t546 - F::cast_from(12.0_f64) * t1819 * t3844 + F::cast_from(3.0_f64) * t1819 * t3847 + F::cast_from(3.0_f64) * t1821 * t3839;
    (t16196, t16199, t16202, t16205)
}
