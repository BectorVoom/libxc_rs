//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1072/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1072<F: Float>(t1169: F, t24330: F, t1188: F, t24375: F, t12397: F, t16706: F, t20283: F, t20285: F, t20287: F, t24230: F, t24234: F, t24238: F, t24242: F, t24246: F, t24250: F) -> (F, F, F) {
    let t24431 = t24330 * t1169;
    let t24436 = t24375 * t1188;
    let t24453 = -t12397 + F::new(0.2283111111111111111e-1) * t16706 + F::new(0.11415555555555555555e-1) * t20283 - F::new(0.34246666666666666665e-1) * t20285 - F::new(0.17123333333333333333e-1) * t20287 + F::new(0.19025925925925925925e-1) * t24230 - F::new(0.68493333333333333331e-1) * t24234 - F::new(0.34246666666666666665e-1) * t24238 + F::new(0.10274e0) * t24242 + F::new(0.10274e0) * t24246 + F::new(0.17123333333333333333e-1) * t24250;
    (t24431, t24436, t24453)
}
