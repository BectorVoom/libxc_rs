//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta856 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2745;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2746;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta856(t17708: f64, t59498: f64, t12916: f64, t21041: f64, t3718: f64, t21165: f64, t12809: f64, t20796: f64, t13045: f64, t5284: f64, t5245: f64, t5457: f64, t1209: f64, t1284: f64, t6695: f64, t20849: f64, t3754: f64, t3781: f64, t6564: f64, t20800: f64, t3302: f64, t13141: f64, t1811: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t72011, t72017, t72064, t72071, t72086, t72143) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2745(t17708, t59498, t12916, t21041, t3718, t21165, t12809, t20796, t13045, t5284, t5245, t5457);
        let (t72267, t72270, t72326, t72329, t72343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2746(t1209, t1284, t6695, t20849, t3754, t3781, t6564, t20800, t3302, t13141, t1811, t460);
    (t72011, t72017, t72064, t72071, t72086, t72143, t72267, t72270, t72326, t72329, t72343)
}
