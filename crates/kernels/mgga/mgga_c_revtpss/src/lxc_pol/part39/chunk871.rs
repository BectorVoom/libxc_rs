//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 871/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk871<F: Float>(t1222: F, t3657: F, t3658: F, t3679: F, t3684: F, t3718: F, t5340: F, t5343: F, t5348: F, t5354: F, t5358: F, t5363: F, t5366: F, t5369: F, t1010: F, t1480: F) -> (F, F) {
    let t5372 = 0.42874018118069736972e-3 * t5340 * t5343 - 0.21437009059034868486e-3 * t3718 * t5348 - 0.21437009059034868486e-3 * t3718 * t5354 - t5358 / 864.0 - t3657 + 0.14291339372689912324e-3 * t3658 - 0.14291339372689912324e-3 * t5363 - 0.14291339372689912324e-3 * t3679 - t5366 / 108.0 - t3684 - t1222 * t5369 / 288.0;
    let t5373 = t1480 * t1010;
    (t5372, t5373)
}
