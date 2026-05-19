//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 675/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk675<F: Float>(t4210: F, t599: F, t1181: F, t7346: F, t1016: F, t137: F) -> (F, F, F, F, F) {
    let t7347 = t599 * t4210;
    let t7348 = t1181 * t7347;
    let t7349 = t7346 * t7348;
    let t7350 = F::cast_from(0.21437009059034868486e-3_f64) * t7349;
    let t7351 = t1016 * t137;
    (t7347, t7348, t7349, t7350, t7351)
}
