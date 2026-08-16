//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2109;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta500(t15158: f64, t4915: f64, t1469: f64, t3075: f64, t4872: f64, t1042: f64, t1011: f64, t1063: f64, t11753: f64, t11756: f64, t11763: f64, t11866: f64, t15782: f64, t15787: f64, t15791: f64, t15796: f64, t3127: f64, t3241: f64, t4892: f64, t4907: f64, t4916: f64, t4920: f64, t1032: f64, t4743: f64, t1040: f64, t1647: f64, t3140: f64, t3149: f64, t11921: f64, t247: f64, t4757: f64, t4837: f64, t1659: f64, t3105: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15804, t15809, t15810, t15811, t15814) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2109(t15158, t4915, t1469, t3075, t4872, t1042, t1011, t1063, t11753, t11756, t11763, t11866, t15782, t15787, t15791, t15796, t3127, t3241, t4892, t4907, t4916, t4920);
        let (t15816, t15817, t15822, t15823, t15827, t15829, t15830) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2110(t1032, t4743, t1040, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
    (t15804, t15809, t15810, t15811, t15814, t15816, t15817, t15822, t15823, t15827, t15829, t15830)
}
