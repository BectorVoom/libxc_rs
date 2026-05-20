//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta500 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2109;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2110;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta500<F: Float>(t15158: F, t4915: F, t1469: F, t3075: F, t4872: F, t1042: F, t1011: F, t1063: F, t11753: F, t11756: F, t11763: F, t11866: F, t15782: F, t15787: F, t15791: F, t15796: F, t3127: F, t3241: F, t4892: F, t4907: F, t4916: F, t4920: F, t1032: F, t4743: F, t1040: F, t1647: F, t3140: F, t3149: F, t11921: F, t247: F, t4757: F, t4837: F, t1659: F, t3105: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15804, t15809, t15810, t15811, t15814) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2109::<F>(t15158, t4915, t1469, t3075, t4872, t1042, t1011, t1063, t11753, t11756, t11763, t11866, t15782, t15787, t15791, t15796, t3127, t3241, t4892, t4907, t4916, t4920);
        let (t15816, t15817, t15822, t15823, t15827, t15829, t15830) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2110::<F>(t1032, t4743, t1040, t1647, t3140, t3149, t11921, t247, t4757, t4837, t1659, t3105);
    (t15804, t15809, t15810, t15811, t15814, t15816, t15817, t15822, t15823, t15827, t15829, t15830)
}
