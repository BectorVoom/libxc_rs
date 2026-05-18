//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 976/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk976<F: Float>(t1294: F, t482: F, t372: F, t371: F, t26904: F, t8937: F, t1276: F, t3596: F, t1245: F) -> (F, F, F, F, F) {
    let t33510 = t482 * t1294;
    let t33511 = t372 * t33510;
    let t33512 = t371 * t33511;
    let t33515 = t8937 * t26904;
    let t33516 = t1276 * t3596;
    let t33517 = t33516 * t1245;
    (t33510, t33512, t33515, t33516, t33517)
}
