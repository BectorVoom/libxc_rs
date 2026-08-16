//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1055;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1056;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta281(t2411: f64, t6075: f64, t11506: f64, t6189: f64, t11144: f64, t5819: f64, t11150: f64, t6093: f64, t689: f64, t6097: f64, t6101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t18865, t18898, t18903, t18908, t18919) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1055(t2411, t6075, t11506, t6189, t11144, t5819, t11150, t6093, t689);
        let t18924 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1056(t6097, t689);
        let t18934 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1057(t6101, t689);
    (t18865, t18898, t18903, t18908, t18919, t18924, t18934)
}
