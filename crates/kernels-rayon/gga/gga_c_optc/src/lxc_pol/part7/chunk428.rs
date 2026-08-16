//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 428/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk428(t135: f64, t2006: f64, t2008: f64, t2011: f64, t2013: f64, t2017: f64, t2021: f64, t2026: f64, t2031: f64, t2037: f64, t2070: f64, t2074: f64, t2082: f64, t2083: f64, t2089: f64, t2093: f64, t628: f64, t636: f64) -> f64 {
    let t2096 = t2006 + 7.0_f64 / 72.0_f64 * t2008 + t2011 * t2013 / 16.0_f64 - t628 * t2017 / 48.0_f64 + 0.54332259311179736592e-2_f64 * t2021 * t2026 + 0.2535505434521721041e-1_f64 * t2031 + 0.21732903724471894636e-1_f64 * t636 * t2037 - 0.27166129655589868296e-2_f64 * t636 * t2070 - 0.27166129655589868296e-2_f64 * t636 * t2074 + t2082 + 0.10142021738086884164e0_f64 * t2083 + 0.5433225931117973659e-1_f64 * t135 * t2089 - 0.10866451862235947318e-1_f64 * t135 * t2093;
    t2096
}
