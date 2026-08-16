//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta463 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1902;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1903;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1904;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta463(t19450: f64, t4900: f64, t3117: f64, t11661: f64, t19501: f64, t3092: f64, t1047: f64, t1063: f64, t12013: f64, t16067: f64, t16089: f64, t19688: f64, t19693: f64, t19697: f64, t19702: f64, t19707: f64, t19718: f64, t3127: f64, t4803: f64, t4808: f64, t4834: f64, t4892: f64, t4899: f64, t6308: f64, t15957: f64, t6266: f64, t16509: f64, t4891: f64, t16584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19721, t19722, t19725, t19726, t19729) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1902(t19450, t4900, t3117, t11661, t19501, t3092, t1047, t1063, t12013, t16067, t16089, t19688, t19693, t19697, t19702, t19707, t19718, t3127, t4803, t4808, t4834, t4892, t4899, t6308);
        let (t19730, t19731, t19738) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1903(t15957, t6266, t3092, t16509, t4891);
        let t19741 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1904(t16584, t4891);
    (t19721, t19722, t19725, t19726, t19729, t19730, t19731, t19738, t19741)
}
