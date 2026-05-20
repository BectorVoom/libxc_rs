//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1252/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1252<F: Float>(t24375: F, t3523: F, t16706: F, t16876: F, t20276: F, t20278: F, t20280: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24265: F, t24267: F, t24272: F, t24275: F) -> (F, F) {
    let t24376 = t24375 * t3523;
    let t24393 = -F::new(0.3883875e1) * t24265 + F::cast_from(0.247573125e0_f64) * t24267 + F::cast_from(0.40256666666666666668e0_f64) * t16706 + F::new(0.27595e0) * t16876 + F::cast_from(0.36793333333333333333e-1_f64) * t24272 + F::new(0.49671e0) * t24275 + F::new(0.5519e-1) * t20276 - F::new(0.33114e0) * t20278 - F::new(0.16557e0) * t20280 + F::cast_from(0.20128333333333333333e0_f64) * t20283 - F::cast_from(0.60385000000000000001e0_f64) * t20285 - F::cast_from(0.30192500000000000001e0_f64) * t20287 + F::cast_from(0.33547222222222222222e0_f64) * t24230 - F::new(0.12077e1) * t24234;
    (t24376, t24393)
}
