//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1084/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1084<F: Float>(t7892: F, t9448: F, t4250: F, t986: F, t25579: F, t493: F, t1339: F, t25722: F, t2897: F, t4461: F, t4348: F, t997: F) -> (F, F, F, F, F, F) {
    let t27007 = t9448 * t7892;
    let t27071 = t4250 * t986;
    let t27082 = t493 * t25579;
    let t27102 = t1339 * t25722;
    let t27114 = t4461 * t2897;
    let t27214 = t997 * t4348;
    (t27007, t27071, t27082, t27102, t27114, t27214)
}
