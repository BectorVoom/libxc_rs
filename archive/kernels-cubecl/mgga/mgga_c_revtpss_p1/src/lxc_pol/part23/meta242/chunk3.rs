//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1411/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1411<F: Float>(t1340: F, t9323: F, t215: F, t681: F, t268: F, t702: F) -> (F, F, F) {
    let t9325 = F::cast_from(0.51947577317044391277e2_f64) * t1340 * t9323;
    let t9326 = t215 * t681;
    let t9329 = F::cast_from(0.71233333333333333332e-1_f64) * t268 * t9326 * t702;
    (t9325, t9326, t9329)
}
