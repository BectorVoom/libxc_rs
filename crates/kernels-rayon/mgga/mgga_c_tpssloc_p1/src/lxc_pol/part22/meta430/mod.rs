//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1760;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1761;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta430(t18188: f64, t19288: f64, t12560: f64, t12561: f64, t12562: f64, t12563: f64, t12564: f64, t12565: f64, t9225: f64, t5385: f64, t604: f64, t5389: f64, t645: f64, t1437: f64, t4021: f64, t5445: f64, t1409: f64, t65: f64, t67: f64, t1864: f64, t3966: f64, t5392: f64, t628: f64, t17635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19289, t19297, t19299, t19310) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1760(t18188, t19288, t12560, t12561, t12562, t12563, t12564, t12565, t9225, t5385, t604, t5389, t645);
        let (t19313, t19318, t19322, t19323, t19326, t19331) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1761(t1437, t4021, t5445, t645, t1409, t65, t67, t1864, t3966, t5392, t628, t17635);
    (t19289, t19297, t19299, t19310, t19313, t19318, t19322, t19323, t19326, t19331)
}
