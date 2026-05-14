//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1230/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1230<F: Float>(t1113: F, t6793: F, t24345: F, t6762: F, t218: F, t5005: F, t18127: F, t200: F, t679: F, t3773: F, t4977: F, t6027: F, t108573: F, t27616: F, t30599: F, t689: F) -> (F, F, F, F, F, F, F) {
    let t123538 = t6793 * t1113;
    let t123543 = t6762 * t24345;
    let t123552 = t218 * t5005;
    let t123560 = t679 * t18127 * t200;
    let t123565 = t3773 * t6027 * t4977;
    let t123579 = t27616 * t108573 * t30599;
    let t123581 = t5005 * t679;
    let t123582 = t123581 * t689;
    (t123538, t123543, t123552, t123560, t123565, t123579, t123582)
}
