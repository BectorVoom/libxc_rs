//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1146;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1147;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta284<F: Float>(t12009: F, t3150: F, t11997: F, t3144: F, t3141: F, t11678: F, t4910: F, t3117: F, t1032: F, t3043: F, t1040: F, t1065: F, t3075: F, t906: F, t1042: F, t1047: F, t1063: F, t1068: F, t11977: F, t11980: F, t11983: F, t11989: F, t11991: F, t11994: F, t11999: F, t12004: F, t12007: F, t3115: F, t3127: F, t3130: F, t3157: F, t3164: F, t11642: F, t11701: F, t11751: F, t11799: F, t11850: F, t11919: F, t11976: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t12010, t12012, t12013, t12016, t12017, t12020, t12021, t12024) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1146::<F>(t12009, t3150, t11997, t3144, t3141, t11678, t4910, t3117, t1032, t3043, t1040, t1065, t3075);
        let (t12025, t12026, t12029) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1147::<F>(t12024, t906, t1042, t1047, t1063, t1068, t11977, t11980, t11983, t11989, t11991, t11994, t11999, t12004, t12007, t12010, t12013, t12017, t12021, t3115, t3127, t3130, t3157, t3164);
        let t12032 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1148::<F>(t11642, t11701, t11751, t11799, t11850, t11919, t11976, t12029);
    (t12012, t12013, t12016, t12017, t12020, t12021, t12024, t12025, t12026, t12032)
}
