//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 627/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk627<F: Float>(t1212: F, t1248: F, t840: F, t871: F, t1234: F, t2755: F, t91: F, t2766: F, t5098: F, t2771: F, t5213: F, t5105: F, t848: F) -> (F, F, F, F, F, F, F) {
    let t5330 = t1212 * t1248;
    let t5332 = t840 * t871 * t5330;
    let t5337 = t1234 * t1234;
    let t5339 = t91 * t2755 * t5337;
    let t5343 = t2766 * t5098;
    let t5346 = t2771 * t5213;
    let t5349 = t848 * t5105;
    (t5330, t5332, t5337, t5339, t5343, t5346, t5349)
}
