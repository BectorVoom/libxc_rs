//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 565/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk565<F: Float>(t1898: F, t226: F, t249: F, t1894: F, t252: F, t214: F, t1880: F, t335: F, t371: F) -> (F, F, F, F, F, F) {
    let t1899 = t226 * t1898;
    let t1900 = t1899 * t249;
    let t1905 = t1894 * t252;
    let t1906 = t214 * t1905;
    let t1907 = t1880 * t1906;
    let t1932 = F::cast_from(1.0_f64) / t371 / t335;
    (t1899, t1900, t1905, t1906, t1907, t1932)
}
