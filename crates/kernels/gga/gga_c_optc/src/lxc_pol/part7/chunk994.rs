//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 994/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk994<F: Float>(t2253: F, t7314: F, t1006: F, t8378: F, t2317: F, t2325: F, t7230: F, t7234: F, t2555: F, t7207: F, t280: F, t881: F, t355: F, t7194: F, t992: F, t2435: F, t2436: F, t7244: F) -> (F, F, F, F, F, F, F, F) {
    let t23476 = t7314 * t2253;
    let t23481 = t1006 * t8378;
    let t23485 = t2325 * t2317;
    let t23490 = t7230 * t7234;
    let t23495 = t2555 * t7207;
    let t23503 = 1.0 / t280 / t881;
    let t23510 = t355 * t7194 * t992;
    let t23513 = t2435 * t2436 * t7244;
    (t23476, t23481, t23485, t23490, t23495, t23503, t23510, t23513)
}
