//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1222/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1222<F: Float>(t11303: F, t20200: F, t27307: F, t27309: F, t33399: F, t8362: F, t19639: F, t34317: F, t1030: F, t3008: F, t33158: F, t34447: F, t3949: F, t9203: F) -> (F, F, F, F, F) {
    let t35115 = t11303 * t20200;
    let t35119 = t27307 * t33399 * t8362 * t27309;
    let t35121 = t34317 * t19639;
    let t35124 = t1030 * t33158 * t3008;
    let t35127 = t9203 * t34447 * t3949;
    (t35115, t35119, t35121, t35124, t35127)
}
