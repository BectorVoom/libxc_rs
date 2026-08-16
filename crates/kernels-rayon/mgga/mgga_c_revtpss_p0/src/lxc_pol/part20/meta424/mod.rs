//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta424 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1591;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1592;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta424(t406: f64, t43822: f64, t12254: f64, t141: f64, t43835: f64, t1145: f64, t43843: f64, t1139: f64, t43908: f64, t3407: f64, t43825: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64, t43806: f64, t43856: f64, t43936: f64, t1179: f64, t1188: f64, t1196: f64, t3515: f64, t3520: f64, t3523: f64, t3794: f64, t12555: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43947, t43950, t43953, t43955, t43957, t43959) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1591(t406, t43822, t12254, t141, t43835, t1145, t43843, t1139, t43908, t3407, t43825, t43886, t43888, t43890, t43892, t43894, t43896, t43899, t43902, t43905);
        let (t43961, t43965, t43966, t43970, t43971, t43977) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1592(t43806, t43856, t43936, t43959, t1179, t1188, t1196, t3515, t3520, t3523, t3794, t12555);
    (t43947, t43950, t43953, t43955, t43957, t43961, t43965, t43966, t43970, t43971, t43977)
}
