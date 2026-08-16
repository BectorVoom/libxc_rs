//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta284 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1146;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1147;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta284(t12009: f64, t3150: f64, t11997: f64, t3144: f64, t3141: f64, t11678: f64, t4910: f64, t3117: f64, t1032: f64, t3043: f64, t1040: f64, t1065: f64, t3075: f64, t906: f64, t1042: f64, t1047: f64, t1063: f64, t1068: f64, t11977: f64, t11980: f64, t11983: f64, t11989: f64, t11991: f64, t11994: f64, t11999: f64, t12004: f64, t12007: f64, t3115: f64, t3127: f64, t3130: f64, t3157: f64, t3164: f64, t11642: f64, t11701: f64, t11751: f64, t11799: f64, t11850: f64, t11919: f64, t11976: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12010, t12012, t12013, t12016, t12017, t12020, t12021, t12024) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1146(t12009, t3150, t11997, t3144, t3141, t11678, t4910, t3117, t1032, t3043, t1040, t1065, t3075);
        let (t12025, t12026, t12029) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1147(t12024, t906, t1042, t1047, t1063, t1068, t11977, t11980, t11983, t11989, t11991, t11994, t11999, t12004, t12007, t12010, t12013, t12017, t12021, t3115, t3127, t3130, t3157, t3164);
        let t12032 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1148(t11642, t11701, t11751, t11799, t11850, t11919, t11976, t12029);
    (t12012, t12013, t12016, t12017, t12020, t12021, t12024, t12025, t12026, t12032)
}
