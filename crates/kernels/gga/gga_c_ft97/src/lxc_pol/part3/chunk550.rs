//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 550/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk550<F: Float>(t1558: F, t4417: F, t1555: F, t89: F, t925: F, t942: F, t1564: F, t446: F, t1571: F, t356: F, t1578: F, t361: F) -> (F, F, F, F, F, F, F, F) {
    let t4418 = t1558 * t4417;
    let t4420 = t89 * t1555 * t4418;
    let t4422 = t925 * t942;
    let t4423 = t1564 * t4422;
    let t4424 = t446 * t4423;
    let t4426 = t1571 * t4417;
    let t4428 = t89 * t356 * t4426;
    let t4431 = F::new(2.0) * t361 + F::new(2.0) * t1578;
    (t4418, t4420, t4422, t4423, t4424, t4426, t4428, t4431)
}
