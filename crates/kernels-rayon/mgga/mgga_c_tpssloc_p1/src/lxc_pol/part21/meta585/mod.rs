//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2317;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2318;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2319;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta585(t16132: f64, t1825: f64, t1352: f64, t19743: f64, t19660: f64, t118: f64, t6330: f64, t794: f64, t12202: f64, t19631: f64, t210: f64, t214: f64, t6347: f64, t3739: f64, t12211: f64, t6353: f64, t213: f64, t1307: f64, t221: f64, t5187: f64, t5196: f64, t12188: f64, t12190: f64, t12194: f64, t12196: f64, t12200: f64, t1315: f64, t16101: f64, t5195: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19756, t19761, t19763, t19767, t19768, t19771) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2317(t16132, t1825, t1352, t19743, t19660, t118, t6330, t794, t12202, t19631, t210, t214);
        let (t19775, t19776, t19779, t19781, t19783, t19787) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2318(t118, t6347, t794, t3739, t12211, t6353, t213, t6330, t1307, t221, t5187, t5196);
        let t19790 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2319(t12188, t12190, t12194, t12196, t12200, t1315, t16101, t19768, t19771, t19776, t19779, t19783, t19787, t5195);
    (t19756, t19761, t19763, t19767, t19771, t19775, t19781, t19783, t19787, t19790)
}
