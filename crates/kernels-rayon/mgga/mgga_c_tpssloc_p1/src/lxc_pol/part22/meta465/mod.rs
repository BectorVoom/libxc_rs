//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1847;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1848;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta465(t20553: f64, t550: f64, t1343: f64, t820: f64, t1799: f64, t6347: f64, t3870: f64, t20489: f64, t20416: f64, t210: f64, t214: f64, t20356: f64, t221: f64, t5196: f64, t12188: f64, t12194: f64, t12196: f64, t12215: f64, t12236: f64, t1315: f64, t16078: f64, t16108: f64, t16119: f64, t19768: f64, t19776: f64, t19779: f64, t19791: f64, t5195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20554, t20556, t20563, t20565, t20568, t20570, t20576, t20582) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1847(t20553, t550, t1343, t820, t1799, t6347, t3870, t20489, t20416, t210, t214, t20356);
        let (t20586, t20594) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1848(t221, t5196, t6347, t12188, t12194, t12196, t12215, t12236, t1315, t16078, t16108, t16119, t19768, t19776, t19779, t19791, t20576, t20582, t5195);
    (t20554, t20556, t20563, t20565, t20568, t20570, t20576, t20582, t20586, t20594)
}
