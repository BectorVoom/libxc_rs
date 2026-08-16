//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta506 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1998;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta506(t1250: f64, t20900: f64, t482: f64, t1042: f64, t19680: f64, t5268: f64, t1247: f64, t1261: f64, t12910: f64, t12956: f64, t17339: f64, t17396: f64, t17505: f64, t20858: f64, t20864: f64, t20868: f64, t20876: f64, t20880: f64, t3708: f64, t3711: f64, t5299: f64, t5354: f64, t6619: f64, t6625: f64, t20823: f64, t5265: f64, t5274: f64, t1774: f64, t3362: f64, t4181: f64, t12787: f64, t12916: f64, t6689: f64, t3718: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20902, t20903, t20906, t20907, t20910) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1998(t1250, t20900, t482, t1042, t19680, t5268, t1247, t1261, t12910, t12956, t17339, t17396, t17505, t20858, t20864, t20868, t20876, t20880, t3708, t3711, t5299, t5354, t6619, t6625);
        let (t20913, t20914, t20917, t20922, t20923, t20926, t20927) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1999(t20823, t5268, t1042, t5265, t5274, t1774, t3362, t4181, t12787, t12916, t6689, t3718);
    (t20902, t20903, t20906, t20907, t20910, t20913, t20914, t20917, t20922, t20923, t20926, t20927)
}
