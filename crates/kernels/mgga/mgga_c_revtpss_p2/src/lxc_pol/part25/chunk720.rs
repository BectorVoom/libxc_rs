//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 720/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk720<F: Float>(t7045: F, t857: F, t7024: F, t7026: F, t7032: F, t7035: F, t7039: F, t7042: F) -> F {
    let t7046 = t7045 * t857;
    let t7048 = -t7024 - t7026 / F::new(48.0) - t7032 + t7035 - F::cast_from(0.42874018118069736972e-3_f64) * t7039 - t7042 - F::cast_from(0.17149607247227894789e-2_f64) * t7046;
    t7048
}
