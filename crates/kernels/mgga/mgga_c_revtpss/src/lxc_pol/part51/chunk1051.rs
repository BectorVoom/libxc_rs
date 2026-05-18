//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1051/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1051<F: Float>(t31912: F, t31965: F, t31908: F, t31949: F, t1032: F, t25698: F, t31919: F, t25638: F, t8513: F, t120304: F, t1982: F, t3316: F) -> (F, F, F, F, F, F) {
    let t120403 = t31912 * t31965;
    let t120406 = t31908 * t31949;
    let t120412 = t25698 * t1032;
    let t120419 = t31919 * t31965;
    let t120425 = t8513 * t25638;
    let t120429 = t1982 * t3316 * t120304;
    (t120403, t120406, t120412, t120419, t120425, t120429)
}
