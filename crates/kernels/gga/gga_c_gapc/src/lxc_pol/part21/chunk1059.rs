//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1059/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1059<F: Float>(t11913: F, t28427: F, t435: F, t9281: F, t1084: F, t3415: F, t11784: F, t9865: F, t11379: F, t11945: F, t28594: F, t11948: F, t30095: F) -> (F, F, F, F, F, F) {
    let t33156 = t11913 * t28427;
    let t33158 = t435 * t9281;
    let t33160 = t1084 * t33158 * t3415;
    let t33162 = t11784 * t9865;
    let t33165 = t28594 * t11379 * t11945;
    let t33167 = t11948 * t30095;
    (t33156, t33158, t33160, t33162, t33165, t33167)
}
