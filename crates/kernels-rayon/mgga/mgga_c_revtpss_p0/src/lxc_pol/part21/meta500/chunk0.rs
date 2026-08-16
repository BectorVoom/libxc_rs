//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2109/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2109(t15158: f64, t4915: f64, t1469: f64, t3075: f64, t4872: f64, t1042: f64, t1011: f64, t1063: f64, t11753: f64, t11756: f64, t11763: f64, t11866: f64, t15782: f64, t15787: f64, t15791: f64, t15796: f64, t3127: f64, t3241: f64, t4892: f64, t4907: f64, t4916: f64, t4920: f64) -> (f64, f64, f64, f64, f64) {
    let t15804 = t4915 * t15158;
    let t15809 = t1469 * t3075;
    let t15810 = t4872 * t15809;
    let t15811 = t1042 * t15810;
    let t15814 = 0.85748036236139473944e-3_f64 * t4892 * t15782 + 0.42874018118069736972e-3_f64 * t4892 * t15787 - 0.57165357490759649296e-3_f64 * t1063 * t15791 - t15796 - 0.42874018118069736972e-3_f64 * t11866 * t4907 + t11753 / 864.0_f64 + t11756 / 648.0_f64 - t11763 / 432.0_f64 + t3241 * t4916 / 27.0_f64 + t1011 * t15804 / 48.0_f64 - 2.0_f64 / 81.0_f64 * t3241 * t4920 - 0.14291339372689912324e-3_f64 * t3127 * t15811;
    (t15804, t15809, t15810, t15811, t15814)
}
