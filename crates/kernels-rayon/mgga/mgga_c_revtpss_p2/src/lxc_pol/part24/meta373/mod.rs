//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1264;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta373(t12839: f64, t1469: f64, t20795: f64, t3626: f64, t6638: f64, t17304: f64, t17340: f64, t17342: f64, t17438: f64, t1791: f64, t20817: f64, t20843: f64, t20847: f64, t20851: f64, t20917: f64, t20927: f64, t20966: f64, t21177: f64, t5331: f64, t5340: f64, t6611: f64, t1715: f64, t21093: f64, t1042: f64, t1774: f64, t5819: f64, t5268: f64, t6573: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24567, t24568, t24569, t24572, t24573, t24587) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1264(t12839, t1469, t20795, t3626, t6638, t17304, t17340, t17342, t17438, t1791, t20817, t20843, t20847, t20851, t20917, t20927, t20966, t21177, t5331, t5340, t6611);
        let (t24604, t24605, t24610, t24611, t24612, t24616) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1265(t1715, t21093, t1042, t1774, t5819, t5268, t6573);
    (t24567, t24568, t24569, t24572, t24573, t24587, t24604, t24605, t24610, t24611, t24612, t24616)
}
