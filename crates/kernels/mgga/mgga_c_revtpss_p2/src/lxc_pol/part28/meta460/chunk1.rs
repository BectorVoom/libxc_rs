//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1758/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1758<F: Float>(t2259: F, t603: F, t48: F, t613: F, t2275: F, t43: F, t239: F, t2251: F, t2258: F, t2269: F, t49: F, t606: F, t6968: F) -> (F, F, F, F, F) {
    let t25120 = t603 * t2259;
    let t25129 = t613 * t48;
    let t25132 = t43 * t2275;
    let t25137 = F::new(88.0) / F::new(9.0) * t239;
    let t25138 = F::new(88.0) / F::new(9.0) * t2269 * t49 - F::new(40.0) / F::new(9.0) * t25129 * t606 + F::new(5.0) / F::new(18.0) * t25132 * t2251 + F::new(5.0) / F::new(6.0) * t6968 * t2258 - t25137;
    (t25120, t25129, t25132, t25137, t25138)
}
