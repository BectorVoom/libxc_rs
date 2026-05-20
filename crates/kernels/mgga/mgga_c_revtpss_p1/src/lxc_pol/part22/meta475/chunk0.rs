//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2175/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2175<F: Float>(t15794: F, t3115: F, t15158: F, t4915: F, t1469: F, t3075: F, t4872: F, t1042: F, t1011: F, t1063: F, t11753: F, t11756: F, t11763: F, t11866: F, t15782: F, t15787: F, t15791: F, t3127: F, t3241: F, t4892: F, t4907: F, t4916: F, t4920: F) -> (F, F, F, F, F) {
    let t15796 = F::cast_from(0.28582678745379824648e-3_f64) * t3115 * t15794;
    let t15804 = t4915 * t15158;
    let t15809 = t1469 * t3075;
    let t15810 = t4872 * t15809;
    let t15811 = t1042 * t15810;
    let t15814 = F::cast_from(0.85748036236139473944e-3_f64) * t4892 * t15782 + F::cast_from(0.42874018118069736972e-3_f64) * t4892 * t15787 - F::cast_from(0.57165357490759649296e-3_f64) * t1063 * t15791 - t15796 - F::cast_from(0.42874018118069736972e-3_f64) * t11866 * t4907 + t11753 / F::new(864.0) + t11756 / F::new(648.0) - t11763 / F::new(432.0) + t3241 * t4916 / F::new(27.0) + t1011 * t15804 / F::new(48.0) - F::new(2.0) / F::new(81.0) * t3241 * t4920 - F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t15811;
    (t15796, t15809, t15810, t15811, t15814)
}
