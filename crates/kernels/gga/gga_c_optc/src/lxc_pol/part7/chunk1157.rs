//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1157/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1157<F: Float>(t7213: F, t8276: F, t2433: F, t2368: F, t7304: F, t7198: F, t984: F, t7330: F, t2329: F, t881: F, t2364: F, t24037: F, t24041: F, t24044: F, t24046: F, t280: F, t287: F, t7268: F, t8: F, t8291: F, t8297: F, t8381: F, t989: F) -> F {
    let t24049 = t7213 * t8276;
    let t24050 = t2433 * t24049;
    let t24052 = t7304 * t2368;
    let t24054 = t984 * t7198;
    let t24058 = t984 * t7330;
    let t24060 = t2329 * t881;
    let t24068 = -t24037 - F::new(32.0) / F::new(3.0) * t2364 * t7268 - F::new(16.0) / F::new(9.0) * t24041 + t24044 + F::new(400.0) / F::new(27.0) * t2433 * t24046 + F::new(200.0) / F::new(81.0) * t24050 - F::new(32.0) / F::new(9.0) * t24052 + F::new(176.0) / F::new(9.0) * t24054 - F::new(16.0) / F::new(3.0) * t8381 * t989 + F::new(20.0) / F::new(27.0) * t24058 - F::new(392000000.0) / F::new(729.0) * t8291 / t280 / t24060 * t8 * t287 * t8297;
    t24068
}
