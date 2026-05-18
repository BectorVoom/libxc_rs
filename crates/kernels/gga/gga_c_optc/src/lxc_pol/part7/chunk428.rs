//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 428/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk428<F: Float>(t135: F, t2006: F, t2008: F, t2011: F, t2013: F, t2017: F, t2021: F, t2026: F, t2031: F, t2037: F, t2070: F, t2074: F, t2082: F, t2083: F, t2089: F, t2093: F, t628: F, t636: F) -> F {
    let t2096 = t2006 + F::new(7.0) / F::new(72.0) * t2008 + t2011 * t2013 / F::new(16.0) - t628 * t2017 / F::new(48.0) + F::new(0.54332259311179736592e-2) * t2021 * t2026 + F::new(0.2535505434521721041e-1) * t2031 + F::new(0.21732903724471894636e-1) * t636 * t2037 - F::new(0.27166129655589868296e-2) * t636 * t2070 - F::new(0.27166129655589868296e-2) * t636 * t2074 + t2082 + F::new(0.10142021738086884164e0) * t2083 + F::new(0.5433225931117973659e-1) * t135 * t2089 - F::new(0.10866451862235947318e-1) * t135 * t2093;
    t2096
}
