//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1400/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1400<F: Float>(t1992: F, t20638: F, t22897: F, t20416: F, t6637: F, t6888: F, t6968: F, t22633: F, t26421: F, t6388: F, t1825: F, t26331: F, t6976: F, t97011: F) -> (F, F, F, F) {
    let t107367 = t1992 * t22897 * t20638;
    let t107377 = t6888 * t6637 * t6968 * t20416;
    let t107381 = t22633 * t22897 * t26421 * t6388;
    let t107385 = t26331 * t6976 * t97011 * t1825;
    (t107367, t107377, t107381, t107385)
}
