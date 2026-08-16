//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 984/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk984<F: Float>(t114797: F, t1484: F, t22986: F, t23270: F, t33448: F, t81591: F, t1888: F, t33457: F, t82159: F, t1880: F, t214: F, t225: F, t258: F, t26653: F) -> (F, F, F, F) {
    let t121367 = t22986 * t23270 * t114797 * t1484;
    let t121371 = t81591 * t33448;
    let t121382 = t1888 * t82159 * t33457;
    let t121391 = t1880 * t214 * t26653 * t225 * t258;
    (t121367, t121371, t121382, t121391)
}
