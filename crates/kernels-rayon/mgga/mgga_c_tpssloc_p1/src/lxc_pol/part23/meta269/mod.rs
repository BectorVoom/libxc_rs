//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta269 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk946;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta269(t1824: f64, t6387: f64, t12250: f64, t1343: f64, t820: f64, t3792: f64, t119: f64, t20416: f64, t210: f64, t12291: f64, t12330: f64, t12335: f64, t1315: f64, t16341: f64, t16350: f64, t19915: f64, t19917: f64, t19933: f64, t3790: f64, t5235: f64, t6417: f64, t20356: f64, t1810: f64, t6347: f64, t11982: f64, t11984: f64, t20354: f64, t20355: f64, t20360: f64, t20361: f64, t20365: f64, t20366: f64, t20370: f64, t9457: f64, t9476: f64, t9484: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t20489, t20490, t20492, t20495, t20497, t20500, t20501, t20508) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk946(t1824, t6387, t12250, t1343, t820, t3792, t119, t20416, t210, t12291, t12330, t12335, t1315, t16341, t16350, t19915, t19917, t19933, t3790, t5235, t6417);
        let (t20512, t20516, t20519) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk947(t119, t20356, t210, t1810, t6347, t11982, t11984, t20354, t20355, t20360, t20361, t20365, t20366, t20370, t9457, t9476, t9484);
    (t20489, t20490, t20492, t20495, t20497, t20500, t20501, t20508, t20512, t20516, t20519)
}
