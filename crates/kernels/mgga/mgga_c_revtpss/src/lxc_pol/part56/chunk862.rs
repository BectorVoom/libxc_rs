//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 862/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk862<F: Float>(t1214: F, t8931: F, t33462: F, t1269: F, t8937: F, t7657: F) -> (F, F, F) {
    let t33470 = t8931 * t1214;
    let t33471 = t33462 * t33470;
    let t33474 = t8937 * t1269;
    let t33477 = t8937 * t7657;
    (t33471, t33474, t33477)
}
