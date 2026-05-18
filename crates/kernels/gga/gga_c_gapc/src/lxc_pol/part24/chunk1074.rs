//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1074/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1074<F: Float>(t11748: F, t2600: F, t11804: F, t11814: F, t2599: F, t11325: F, t3402: F, t9934: F, t11872: F, t9723: F, t10072: F, t11930: F) -> (F, F, F, F, F, F) {
    let t33197 = t11748 * t2600;
    let t33200 = t11814 * t11804 * t2599;
    let t33202 = t3402 * t11325;
    let t33203 = t33202 * t9934;
    let t33205 = t11872 * t9723;
    let t33209 = t11930 * t10072;
    (t33197, t33200, t33202, t33203, t33205, t33209)
}
