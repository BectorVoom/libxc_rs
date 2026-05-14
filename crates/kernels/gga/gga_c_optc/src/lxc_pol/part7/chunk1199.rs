//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1199/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1199<F: Float>(t26280: F, t26284: F, t26293: F, t26296: F, t26304: F, t26311: F, t26324: F, t26394: F, t26396: F, t26398: F, t26406: F, t26409: F, t26412: F, t26415: F, t1051: F, t25: F, t26302: F) -> (F, F) {
    let t26417 = -0.79724444444444444444e0 * t26324 - 0.13145066666666666666e1 * t26394 + 0.21908444444444444444e0 * t26396 - 0.28483875e1 * t26398 + 0.23917333333333333333e1 * t26280 - 0.71752000000000000002e1 * t26284 - 0.59793333333333333333e0 * t26293 + 0.71752e1 * t26296 + 0.17938e1 * t26304 - 0.15944888888888888889e1 * t26311 + 0.13145066666666666666e1 * t26406 - 0.98587999999999999998e0 * t26409 - 0.82156666666666666668e-1 * t26412 + 0.197176e1 * t26415;
    let t26419 = t25 * t1051 * t26302;
    (t26417, t26419)
}
