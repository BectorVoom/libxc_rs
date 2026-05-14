//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 471/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk471<F: Float>(t2435: F, t2439: F, t2502: F, t2504: F, t2509: F, t2511: F, t701: F, t682: F) -> (F, F, F) {
    let t2576 = -0.42198333333333333333e0 * t2502 + 0.84396666666666666666e0 * t2504 + 0.39862222222222222223e0 * t2435 + 0.68258333333333333333e-1 * t2509 + 0.13651666666666666667e0 * t2511 + 0.13692777777777777778e0 * t2439;
    let t2577 = t2576 * t701;
    let t2579 = 1.0 * t682 * t2577;
    (t2576, t2577, t2579)
}
