//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1169/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1169<F: Float>(t25383: F, t26475: F, t26511: F, t7067: F, t7415: F, t93126: F, t95538: F, t95542: F, t95543: F, t95548: F, t95551: F, t95553: F, t95556: F, t95562: F, t95567: F, t95569: F, t95572: F) -> F {
    let t95574 = -F::new(0.15421710918628844643e0) * t95538 - t95542 - F::new(0.38554277296572111609e-1) * t95543 - t95548 - F::new(0.13010442282307799193e1) * t7067 * t26475 - F::new(0.28912093960683998208e-1) * t95551 - F::new(0.86736281882051994623e-1) * t95553 + F::new(0.16463622957338778996e-1) * t95556 - F::new(0.26020884564615598386e1) * t25383 * t26511 - F::new(0.19514881078765566038e-2) * t95562 + F::new(0.26020884564615598386e1) * t93126 * t7415 + t95567 + t95569 - F::new(0.43368140941025997312e-1) * t95572;
    t95574
}
