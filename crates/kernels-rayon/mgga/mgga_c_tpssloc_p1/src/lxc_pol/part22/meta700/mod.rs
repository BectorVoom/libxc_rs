//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta700 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2284;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta700(t1174: f64, t18206: f64, t44562: f64, t1227: f64, t13969: f64, t18958: f64, t248: f64, t45046: f64, t5971: f64, t15643: f64, t5005: f64, t1009: f64, t18571: f64, t1011: f64, t1212: f64, t3032: f64, t65253: f64, t3505: f64, t3514: f64, t15495: f64, t4997: f64, t15492: f64, t5019: f64, t15591: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65914, t65920, t65935, t65952, t65955) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2284(t1174, t18206, t44562, t1227, t13969, t18958, t248, t45046, t5971, t15643, t5005, t1009, t18571);
        let (t65957, t65963, t65966, t65992, t65994, t65996) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2285(t1011, t1212, t65955, t3032, t65253, t3505, t3514, t15495, t4997, t15492, t5019, t15591);
    (t65914, t65920, t65935, t65952, t65955, t65957, t65963, t65966, t65992, t65994, t65996)
}
