//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1289/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1289<F: Float>(t122820: F, t128539: F, t128543: F, t128552: F, t128554: F, t128557: F, t128560: F, t128562: F, t128572: F, t128574: F, t128577: F, t128867: F, t1453: F, t28588: F, t32822: F, t34788: F, t8111: F) -> F {
    let t131005 = -F::new(3.0) * t122820 * t28588 + t1453 * t34788 - t32822 * t8111 - t128539 - t128543 - t128552 - t128554 - t128557 - t128560 - t128562 + t128572 - t128574 + t128577 + t128867;
    t131005
}
