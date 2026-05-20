//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 723/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk723<F: Float>(t1416: F, t7271: F, t7251: F, t7253: F, t7258: F, t7261: F, t7265: F, t7268: F) -> F {
    let t7272 = t7271 * t1416;
    let t7274 = -t7251 - t7253 / F::new(48.0) - t7258 + t7261 - F::cast_from(0.42874018118069736972e-3_f64) * t7265 - t7268 - F::cast_from(0.17149607247227894789e-2_f64) * t7272;
    t7274
}
