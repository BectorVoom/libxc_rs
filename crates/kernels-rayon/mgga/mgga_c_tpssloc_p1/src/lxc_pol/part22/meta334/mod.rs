//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1526;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta334(t3788: f64, t836: f64, t1336: f64, t5252: f64, t3777: f64, t5245: f64, t1834: f64, t3787: f64, t225: f64, t5319: f64, t5217: f64, t1390: f64, t5356: f64, t112: f64, t5363: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16397, t16398) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1526(t3788, t836, t1336);
        let (t16400, t16401, t16428, t16439, t16460, t16497, t16521) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1527(t16398, t5252, t3777, t5245, t1834, t3787, t225, t5319, t5217, t1390, t5356, t112, t5363);
    (t16397, t16398, t16400, t16401, t16428, t16439, t16460, t16497, t16521)
}
