//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 763/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk763<F: Float>(t1008: F, t7312: F, t1007: F, t2466: F, t2472: F, t3802: F, t845: F, t1002: F, t1015: F, t2337: F, t2360: F, t2364: F, t2551: F, t2563: F, t2569: F, t2822: F, t3980: F, t7259: F, t7263: F, t7268: F, t7276: F, t7279: F, t7281: F, t7285: F, t7288: F, t7295: F, t7301: F, t7304: F, t7308: F, t960: F, t999: F) -> (F, F, F, F, F) {
    let t7313 = t1008 * t7312;
    let t7314 = t1007 * t7313;
    let t7318 = t2472 * t2466 * t3802;
    let t7320 = F::cast_from(0.51947267698127589897e2_f64) * t845 * t7318;
    let t7321 = F::new(2.0) / F::new(3.0) * t2360 * t2551 + F::new(14.0) / F::new(27.0) * t999 * t7259 + t7263 * t1002 / F::new(2.0) + t999 * t7268 - F::cast_from(0.77534644304710291488e-2_f64) * t3980 * t960 * t2569 * t2822 - t7276 / F::new(9.0) - t7279 / F::new(3.0) + t7281 / F::new(3.0) - t2360 * t2563 + F::new(44.0) / F::new(9.0) * t7285 * t1002 - F::new(8.0) / F::new(9.0) * t7288 + F::new(8.0) / F::new(3.0) * t2364 * t2563 - F::new(16.0) / F::new(9.0) * t2364 * t2551 + F::new(2.0) / F::new(9.0) * t7295 - F::new(4.0) / F::new(3.0) * t999 * t7301 - F::new(8.0) / F::new(3.0) * t7304 * t1002 + F::new(20000.0) / F::new(27.0) * t7308 * t2337 + F::new(34100.0) / F::new(243.0) * t7314 * t1015 - t7320;
    (t7313, t7314, t7318, t7320, t7321)
}
