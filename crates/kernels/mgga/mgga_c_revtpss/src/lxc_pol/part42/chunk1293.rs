//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1293/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1293<F: Float>(t114: F, t21876: F, t655: F, t10201: F, t10202: F, t13448: F, t13451: F, t13453: F, t21818: F, t21821: F, t21824: F, t21827: F, t21830: F, t69: F) -> (F,) {
    let t115 = 1.0 < t114;
    let t21877 = t655 * t21876;
    let t21880 = -t10201 - 11.0 / 9.0 * t10202 - 22.0 / 9.0 * t13448 - t13451 + t13453 - 2.0 / 3.0 * t21818 - 3.0 / 4.0 * t69 * t21821 + t69 * t21824 / 2.0 + t21827 / 3.0 + t69 * t21830 / 4.0 - t69 * t21877 / 8.0;
    let t21881 = piecewise3(t115, 0.0, t21880);
    (t21881,)
}
