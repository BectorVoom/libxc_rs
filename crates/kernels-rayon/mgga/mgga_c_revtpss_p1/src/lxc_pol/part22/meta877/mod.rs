//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta877 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3043;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3044;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta877(t14939: f64, t233: f64, t689: f64, t869: f64, t10069: f64, t14588: f64, t10518: f64, t14606: f64, t231: f64, t2782: f64, t2783: f64, t51380: f64, t10073: f64, t14504: f64, t10547: f64, t14568: f64, t50560: f64, t2797: f64, t18632: f64, t836: f64, t10529: f64, t14602: f64, t2482: f64, t2811: f64, t4423: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t51505, t51507, t51512, t51519) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3043(t14939, t233, t689, t869, t10069, t14588, t10518, t14606, t231, t2782, t2783, t51380);
        let (t51521, t51523, t51527, t51529, t51531, t51535) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3044(t10073, t14504, t10547, t14568, t231, t50560, t2782, t2797, t18632, t836, t10529, t14602, t2482, t2811, t4423);
    (t51505, t51507, t51512, t51519, t51521, t51523, t51527, t51529, t51531, t51535)
}
