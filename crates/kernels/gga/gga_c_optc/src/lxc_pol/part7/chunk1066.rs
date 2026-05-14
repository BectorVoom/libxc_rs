//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1066/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1066<F: Float>(t2326: F, t2328: F, t7228: F, t8: F, t2436: F, t7299: F, t8385: F, t7192: F, t996: F, t997: F, t2343: F, t2351: F, t355: F, t2320: F, t2352: F, t1002: F, t2344: F, t23563: F, t24146: F, t24151: F, t24155: F, t2433: F, t2549: F, t2563: F, t7219: F, t7224: F, t7285: F, t914: F, t999: F) -> (F,) {
    let t24160 = t2326 * t2328 * t7228 * t8;
    let t24164 = t8385 * t2436 * t7299;
    let t24170 = t996 * t997 * t7192;
    let t24178 = t355 * t2343 * t2351;
    let t24180 = t2320 * t2352;
    let t24184 = 80000.0 / 243.0 * t24146 + 5600.0 / 729.0 * t2433 * t24151 - 80000.0 / 81.0 * t7219 * t24155 - 1520000.0 / 243.0 * t24160 * t7224 - 1600.0 / 81.0 * t2433 * t24164 - 176.0 / 9.0 * t7285 * t2563 - 2464.0 / 81.0 * t24170 * t1002 + 2.0 / 3.0 * t999 * t914 * t2549 * t23563 - 176.0 / 27.0 * t24178 - 2.0 / 3.0 * t24180 + 88.0 / 3.0 * t2320 * t2344;
    (t24184,)
}
