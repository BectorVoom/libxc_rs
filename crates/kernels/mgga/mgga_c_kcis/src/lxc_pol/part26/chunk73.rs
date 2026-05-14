//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 73/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk73<F: Float>(t174: F, t176: F, t37: F, t173: F, t44: F, t41: F, zeta_threshold: F) -> (F, F) {
    let t175 = t174 <= zeta_threshold;
    let t178 = piecewise3(t175, t37, t176 * t174);
    let t180 = (t173 + t178 - 2.0) * t44;
    let t183 = piecewise3(2.0 <= zeta_threshold, t37, 2.0 * t41);
    let t185 = piecewise3(0.0 <= zeta_threshold, t37, 0.0);
    let t187 = (t183 + t185 - 2.0) * t44;
    (t180, t187)
}
