//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta444 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1794;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1795;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta444(t16132: f64, t1825: f64, t1352: f64, t19743: f64, t19660: f64, t118: f64, t6330: f64, t794: f64, t12202: f64, t19631: f64, t210: f64, t214: f64, t6347: f64, t3739: f64, t12211: f64, t6353: f64, t213: f64, t1307: f64, t221: f64, t5187: f64, t5196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19756, t19761, t19763, t19767, t19768, t19771) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1794(t16132, t1825, t1352, t19743, t19660, t118, t6330, t794, t12202, t19631, t210, t214);
        let (t19775, t19776, t19779, t19781, t19783, t19787) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1795(t118, t6347, t794, t3739, t12211, t6353, t213, t6330, t1307, t221, t5187, t5196);
    (t19756, t19761, t19763, t19767, t19768, t19771, t19775, t19776, t19779, t19781, t19783, t19787)
}
