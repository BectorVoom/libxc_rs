//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1569;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1570;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta532(t22890: f64, t9962: f64, t13845: f64, t22841: f64, t73731: f64, t9818: f64, t13847: f64, t1883: f64, t73856: f64, t9816: f64, t22895: f64, t125: f64, t22813: f64, t22857: f64, t22809: f64, t22953: f64, t6843: f64, t9994: f64, t6869: f64, t22829: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t85516, t85532, t85543, t85545, t85548) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1569(t22890, t9962, t13845, t22841, t73731, t9818, t13847, t1883, t73856, t9816, t22895, t125, t22813);
        let (t85553, t85563, t85609, t85638, t85648, t85652) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1570(t125, t22857, t22809, t22953, t6843, t9994, t6869, t73731, t9816, t9818, t22829, t9962);
    (t85516, t85532, t85543, t85545, t85548, t85553, t85563, t85609, t85638, t85648, t85652)
}
