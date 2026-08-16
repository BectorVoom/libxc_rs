//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1074/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1074<F: Float>(t1453: F, t32107: F, t32109: F, t32619: F, t32620: F, t32627: F, t32628: F, t32632: F, t32634: F, t32635: F, t32637: F, t32663: F, t7539: F, t8463: F, t8764: F, t8897: F) -> F {
    let t33261 = t1453 * t8897 - t7539 * t8764 - t32107 - t32109 - t32619 - t32620 + t32627 + t32628 + t32632 - t32634 - t32635 - t32637 - t32663 - t8463;
    t33261
}
