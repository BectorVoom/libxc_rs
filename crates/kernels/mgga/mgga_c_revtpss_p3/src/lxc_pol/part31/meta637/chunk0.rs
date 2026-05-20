//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2092/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2092<F: Float>(t15654: F, t1976: F, t27708: F, t3336: F, t11108: F, t7840: F, t33: F, t41154: F, t1711: F, t2411: F, t28150: F, t6973: F) -> (F, F, F, F, F, F) {
    let t100760 = t15654 * t1976;
    let t100802 = t27708 * t3336;
    let t100806 = t7840 * t11108;
    let t100981 = t41154 * t33;
    let t100987 = t2411 * t1711;
    let t101211 = t6973 * t28150;
    (t100760, t100802, t100806, t100981, t100987, t101211)
}
