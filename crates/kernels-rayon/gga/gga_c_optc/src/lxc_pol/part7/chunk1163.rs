//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1163/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1163(t2436: f64, t7299: f64, t8385: f64, t7192: f64, t996: f64, t997: f64, t2343: f64, t2351: f64, t355: f64, t2320: f64, t2352: f64, t1002: f64, t2344: f64, t23563: f64, t24146: f64, t24151: f64, t24155: f64, t24160: f64, t2433: f64, t2549: f64, t2563: f64, t7219: f64, t7224: f64, t7285: f64, t914: f64, t999: f64) -> f64 {
    let t24164 = t8385 * t2436 * t7299;
    let t24170 = t996 * t997 * t7192;
    let t24178 = t355 * t2343 * t2351;
    let t24180 = t2320 * t2352;
    let t24184 = 80000.0_f64 / 243.0_f64 * t24146 + 5600.0_f64 / 729.0_f64 * t2433 * t24151 - 80000.0_f64 / 81.0_f64 * t7219 * t24155 - 1520000.0_f64 / 243.0_f64 * t24160 * t7224 - 1600.0_f64 / 81.0_f64 * t2433 * t24164 - 176.0_f64 / 9.0_f64 * t7285 * t2563 - 2464.0_f64 / 81.0_f64 * t24170 * t1002 + 2.0_f64 / 3.0_f64 * t999 * t914 * t2549 * t23563 - 176.0_f64 / 27.0_f64 * t24178 - 2.0_f64 / 3.0_f64 * t24180 + 88.0_f64 / 3.0_f64 * t2320 * t2344;
    t24184
}
