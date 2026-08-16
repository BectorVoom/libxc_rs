//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 725/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk725<F: Float>(t3475: F, t426: F, t434: F) -> (F, F, F, F) {
    let t3476 = F::cast_from(1.0_f64) / t3475;
    let t3477 = t426 * t3476;
    let t3478 = t434 * t434;
    let t3479 = F::cast_from(1.0_f64) / t3478;
    (t3476, t3477, t3478, t3479)
}
