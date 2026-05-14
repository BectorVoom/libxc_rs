//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1045/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1045<F: Float>(t127439: F, t127442: F, t127443: F, t127447: F, t127449: F, t127453: F, t127455: F, t127459: F, t127462: F, t127465: F, t127468: F, t127472: F, t127475: F, t1461: F, t1918: F, t32354: F, t32377: F, t33992: F, t5802: F, t8607: F) -> (F,) {
    let t127477 = 3.0 * t1461 * t33992 + 3.0 * t1918 * t32354 + 6.0 * t5802 * t8607 + 6.0 * t127439 + t127442 + 12.0 * t127443 + t127447 + t127449 + t127453 + t127455 + t127459 + t127462 + t127465 + t127468 + t127472 + 12.0 * t127475 + t32377;
    (t127477,)
}
