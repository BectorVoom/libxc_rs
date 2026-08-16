//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta494 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1993;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1994;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1995;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta494(t120: f64, t5187: f64, t1352: f64, t3805: f64, t3851: f64, t5301: f64, t1810: f64, t210: f64, t3734: f64, t3856: f64, t3793: f64, t5248: f64, t5249: f64, t3802: f64, t5234: f64, t3788: f64, t836: f64, t1336: f64, t5252: f64, t3777: f64, t5245: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16364, t16366, t16370, t16379, t16383, t16387) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1993(t120, t5187, t1352, t3805, t3851, t5301, t1810, t210, t3734, t3856, t3793, t5248, t5249);
        let (t16391, t16394) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1994(t3793, t3805, t5301, t3802, t5234);
        let (t16397, t16398) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1995(t3788, t836, t1336);
        let (t16400, t16401) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1996(t16398, t5252, t3777, t5245);
    (t16364, t16366, t16370, t16379, t16383, t16387, t16391, t16394, t16397, t16398, t16400, t16401)
}
