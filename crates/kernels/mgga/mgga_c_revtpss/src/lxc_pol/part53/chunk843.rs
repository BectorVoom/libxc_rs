//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 843/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk843<F: Float>(t28076: F, t72: F, t1927: F, t6977: F, t7715: F, t6973: F, t7719: F, t4237: F, t76: F, t1926: F, t13269: F, t38: F, t1497: F, t640: F, t77: F, t4241: F, t84: F) -> (F, F, F, F, F, F, F, F) {
    let t28077 = t28076 * t72;
    let t28078 = t28077 * t1927;
    let t28081 = t7715 * t6977;
    let t28086 = t6973 * t7719;
    let t28089 = t76 * t4237;
    let t28090 = t1926 * t28089;
    let t28093 = t13269 * t38;
    let t28104 = t640 * t1497;
    let t28105 = t77 * t28104;
    let t28108 = t84 * t4241;
    (t28078, t28081, t28086, t28089, t28090, t28093, t28105, t28108)
}
