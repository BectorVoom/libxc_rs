//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1905;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1906;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta529<F: Float>(t28076: F, t72: F, t1927: F, t6977: F, t7715: F, t6973: F, t7719: F, t4237: F, t76: F, t1926: F, t13269: F, t38: F, t1497: F, t640: F, t77: F, t4241: F, t84: F, t1470: F, t2242: F, t1923: F, t1928: F, t25106: F, t6954: F, t6958: F, t6974: F, t6978: F, t7702: F, t7706: F, t7716: F, t7720: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28077, t28078, t28081, t28086, t28089, t28090, t28093) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1905::<F>(t28076, t72, t1927, t6977, t7715, t6973, t7719, t4237, t76, t1926, t13269, t38);
        let (t28105, t28109, t28112, t28115) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1906::<F>(t1497, t640, t77, t4241, t84, t1470, t2242, t1923, t1928, t25106, t28078, t28081, t28086, t28090, t28093, t6954, t6958, t6974, t6978, t7702, t7706, t7716, t7720);
    (t28077, t28078, t28081, t28086, t28089, t28090, t28093, t28105, t28109, t28112, t28115)
}
