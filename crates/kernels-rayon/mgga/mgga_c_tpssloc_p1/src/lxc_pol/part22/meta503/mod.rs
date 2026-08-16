//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1941;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1942;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta503(t1670: f64, t5988: f64, t1118: f64, t3313: f64, t14838: f64, t5989: f64, t1703: f64, t18915: f64, t4869: f64, t6098: f64, t4748: f64, t5999: f64, t4764: f64, t4723: f64, t5398: f64, t3297: f64, t136: f64, t4728: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21723, t21724, t21726, t21728, t21730, t21732, t21739) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1941(t1670, t5988, t1118, t3313, t14838, t5989, t1703, t18915, t4869, t6098, t4748, t5999);
        let (t21741, t21745) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1942(t4764, t5999, t4723, t5398);
        let (t21746, t21747, t21749) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1943(t21745, t3297, t136, t4728, t5398);
    (t21723, t21724, t21726, t21728, t21730, t21732, t21739, t21741, t21745, t21746, t21747, t21749)
}
