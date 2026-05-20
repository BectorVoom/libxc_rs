//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 957/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk957<F: Float>(t5351: F, t5458: F, t3766: F, t487: F, t460: F, t3302: F, t3603: F) -> (F, F, F, F) {
    let t5459 = t5351 * t5458;
    let t5462 = t3766 * t487;
    let t5463 = t460 * t5462;
    let t5464 = t3302 * t3603;
    (t5459, t5462, t5463, t5464)
}
