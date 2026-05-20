//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1499/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1499<F: Float>(t10014: F, t4104: F, t268: F, t4056: F, t543: F, t675: F, t4101: F, t555: F, t5744: F) -> (F, F, F, F) {
    let t10015 = t10014 * t4104;
    let t10019 = t268 * t675 * t4056 * t543;
    let t10020 = t4101 * t10019;
    let t10022 = t5744 * t555;
    (t10015, t10019, t10020, t10022)
}
