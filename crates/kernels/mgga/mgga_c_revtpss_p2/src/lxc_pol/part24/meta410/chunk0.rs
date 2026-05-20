//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1352/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1352<F: Float>(t2851: F, t25273: F, t268: F, t271: F, t11852: F, t159: F, t273: F, t270: F, t276: F, t39484: F, t2922: F, t275: F) -> (F, F, F, F, F, F, F, F) {
    let t41295 = t2851 * t2851;
    let t41296 = F::new(1.0) / t41295;
    let t41306 = t268 * t25273 * t271;
    let t41307 = F::cast_from(0.31310740740740740741e1_f64) * t41306;
    let t41329 = F::new(280.0) / F::new(81.0) * t41306;
    let t41339 = t159 * t11852;
    let t41382 = F::powf(t273, -F::new(0.25e1));
    let t41401 = F::new(1.0) / t276 / t39484 / t270 / F::new(96.0);
    let t41497 = t2922 * t2922;
    let t41499 = t275 / t41497;
    (t41296, t41306, t41307, t41329, t41339, t41382, t41401, t41499)
}
