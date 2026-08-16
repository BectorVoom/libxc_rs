//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta711 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2468;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta711(t47944: f64, t14078: f64, t2470: f64, t3915: f64, t13735: f64, t2435: f64, t10115: f64, t1900: f64, t14189: f64, t22: f64, t46389: f64, t543: f64, t5735: f64, t1432: f64, t5763: f64, t9288: f64, t10069: f64, t14124: f64, t14129: f64, t14231: f64, t10139: f64, t136: f64, t2457: f64, t5659: f64, t14202: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47945, t47948, t47953, t47961, t47964, t47967) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2468(t47944, t14078, t2470, t3915, t13735, t2435, t10115, t1900, t14189, t22, t46389, t543, t5735);
        let (t47971, t47979, t47981, t47985, t48004, t48005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2469(t1432, t5763, t9288, t10069, t14124, t14129, t14231, t10139, t136, t2457, t5659, t14202, t9303);
    (t47945, t47948, t47953, t47961, t47964, t47967, t47971, t47979, t47981, t47985, t48004, t48005)
}
