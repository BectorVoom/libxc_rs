//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 915/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk915<F: Float>(t11459: F, t11470: F, t14670: F, t14739: F, t14744: F, t14753: F, t14758: F, t17176: F, t17181: F, t17186: F, t17191: F, t17196: F, t17201: F, t2668: F, t3884: F, t8231: F, t913: F, t940: F, t953: F) -> F {
    let t17206 = F::new(0.25190352229182098644e-1) * t953 * t17176 + F::new(0.1559479530529405812e2) * t14670 - F::new(0.30228422675018518374e-1) * t953 * t17181 + F::new(0.11360101276506094136e1) * t913 * t17186 + F::new(0.5848048239485271795e1) * t940 * t17191 - F::new(0.57954409931925052365e-1) * t14739 + F::new(0.38636273287950034909e-1) * t14744 - F::new(0.4395493670620718481e3) * t3884 * t17196 - F::new(0.75734008510040627575e0) * t11459 - F::new(0.389869882632351453e1) * t11470 + t8231 - F::new(0.15486228121497046737e2) * t2668 * t17201 - F::new(0.4395493670620718481e3) * t14753 + F::new(0.8790987341241436962e3) * t14758;
    t17206
}
