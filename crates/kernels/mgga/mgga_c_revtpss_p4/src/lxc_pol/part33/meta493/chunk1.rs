//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1794/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1794<F: Float>(t25953: F, t7289: F, t3974: F, t7259: F, t2482: F, t27: F, t7269: F) -> (F, F, F) {
    let t25955 = F::cast_from(0.17135234354032049604e-1_f64) * t7289 * t25953;
    let t25969 = t7259 * t3974;
    let t25970 = F::cast_from(0.27104001498285508387e-3_f64) * t25969;
    let t25972 = t2482 * t7269 * t27;
    (t25955, t25970, t25972)
}
