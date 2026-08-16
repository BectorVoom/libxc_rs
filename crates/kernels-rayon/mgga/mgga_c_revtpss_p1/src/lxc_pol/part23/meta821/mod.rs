//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta821 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2671;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2672;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta821(t11999: f64, t19826: f64, t11262: f64, t3150: f64, t6307: f64, t11710: f64, t19725: f64, t4892: f64, t15669: f64, t16088: f64, t380: f64, t1045: f64, t4186: f64, t1058: f64, t19858: f64, t15688: f64, t16509: f64, t19869: f64, t3201: f64, t6318: f64, t1011: f64, t15987: f64, t18926: f64, t18930: f64, t15689: f64, t19985: f64, t53405: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66024, t66029, t66043, t66047, t66066) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2671(t11999, t19826, t11262, t3150, t6307, t11710, t19725, t4892, t15669, t16088, t380, t1045, t4186);
        let (t66093, t66114, t66139, t66141, t66155, t66158, t66176) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2672(t1058, t19858, t15688, t16509, t19869, t3201, t6318, t1011, t15987, t18926, t18930, t15689, t19985, t53405);
    (t66024, t66029, t66043, t66047, t66066, t66093, t66114, t66139, t66141, t66155, t66158, t66176)
}
