//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 773/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk773<F: Float>(t103: F, t7165: F, t379: F, t8217: F, t1307: F, t452: F, t5750: F, t1871: F, t499: F, t1882: F, t7222: F, t7235: F) -> (F, F, F, F, F, F, F) {
    let t32494 = t103 * t7165;
    let t32495 = t32494 * t379;
    let t32496 = t8217 * t32495;
    let t32500 = t452 * t5750 * t1307;
    let t32504 = t1871 * t499 * t7165;
    let t32508 = F::new(2.0) / F::new(9.0) * t1882 * t7222;
    let t32510 = t1882 * t7235 / F::new(9.0);
    (t32494, t32495, t32496, t32500, t32504, t32508, t32510)
}
