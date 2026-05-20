//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1987/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1987<F: Float>(t2411: F, t605: F, t198: F, t206: F, t7086: F, t25373: F, t25392: F, t25386: F, t25372: F, t2435: F, t25352: F, t11015: F, t7018: F) -> (F, F, F, F, F, F) {
    let t92790 = t2411 * t605;
    let t92819 = t198 * t206 * t7086;
    let t92837 = t25373 * t25392;
    let t92838 = t25386 * t92837;
    let t92843 = t25372 * t92837;
    let t92858 = t2435 * t25352;
    let t92861 = F::cast_from(0.30356481678079769392e-1_f64) * t7018 * t11015;
    (t92790, t92819, t92838, t92843, t92858, t92861)
}
