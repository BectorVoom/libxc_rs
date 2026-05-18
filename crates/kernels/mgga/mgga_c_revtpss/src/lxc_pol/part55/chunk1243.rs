//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1243/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1243<F: Float>(t8697: F, t8995: F, t28199: F, t122647: F, t27154: F, t26399: F, t7735: F, t28658: F, t27137: F, t7359: F, t28711: F, t8634: F) -> (F, F, F, F, F, F) {
    let t128529 = t8697 * t8995;
    let t128531 = F::new(2.0) * t128529 * t28199;
    let t128533 = F::new(3.0) * t122647 * t27154;
    let t128535 = F::new(2.0) * t26399 * t7735;
    let t128537 = F::new(2.0) * t28658 * t7735;
    let t128539 = F::new(2.0) * t7359 * t27137;
    let t128543 = F::new(2.0) * t8634 * t28711;
    (t128531, t128533, t128535, t128537, t128539, t128543)
}
