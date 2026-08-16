//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1066/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1066<F: Float>(t1040: F, t19696: F, t16509: F, t4891: F, t16584: F, t19463: F, t366: F, t11710: F, t6267: F, t3091: F, t3172: F, t6311: F) -> (F, F, F, F, F, F, F) {
    let t19697 = t19696 * t1040;
    let t19738 = t16509 * t4891;
    let t19741 = t16584 * t4891;
    let t19773 = t19463 * t366;
    let t19785 = t11710 * t6267;
    let t19786 = t3091 * t19785;
    let t19826 = t3172 * t6311;
    (t19697, t19738, t19741, t19773, t19785, t19786, t19826)
}
