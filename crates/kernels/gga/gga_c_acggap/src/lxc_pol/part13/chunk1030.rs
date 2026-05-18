//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1030/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1030<F: Float>(t1988: F, t8541: F, t30811: F, t4908: F, t4680: F, t7493: F, t8648: F, t1421: F, t1992: F, t30827: F, t7842: F, t1165: F, t4752: F, t7351: F, t7575: F) -> (F, F, F, F, F) {
    let t34170 = t1988 * t8541;
    let t34171 = F::new(0.10718504529517434243e-2) * t34170;
    let t34172 = t30811 * t4908;
    let t34173 = F::new(0.68598428988911579156e-2) * t34172;
    let t34175 = t7493 * t4680 * t8648;
    let t34176 = F::new(0.10718504529517434243e-2) * t34175;
    let t34179 = t30827 * t7842 * t1992 * t1421;
    let t34183 = t7575 * t1165 * t7351 * t4752;
    (t34171, t34173, t34176, t34179, t34183)
}
