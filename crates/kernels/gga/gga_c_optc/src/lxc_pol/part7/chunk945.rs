//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 945/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk945<F: Float>(t43: F, t22302: F, t1871: F, t1906: F, t40: F, t768: F, t97: F, t1884: F, t1885: F, t1891: F, t22015: F, t22021: F, t22028: F, t47: F, t6541: F, t6713: F, t6716: F, zeta_threshold: F) -> (F, F, F) {
    let t44 = t43 <= zeta_threshold;
    let t22303 = 4.0 * t22302;
    let t22305 = t40 * t1906 * t1871;
    let t22306 = 6.0 * t22305;
    let t22308 = 1.0 / t97 / t768;
    let t22321 = piecewise3(t44, 0.0, 40.0 / 81.0 * t22308 * t22015 - 16.0 / 9.0 * t6713 * t1885 * t1891 + 4.0 / 3.0 * t1884 * t22021 + 16.0 / 9.0 * t6716 * t6541 + 4.0 / 3.0 * t47 * t22028);
    (t22303, t22306, t22321)
}
