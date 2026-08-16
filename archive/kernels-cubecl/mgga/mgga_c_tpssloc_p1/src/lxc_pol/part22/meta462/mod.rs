//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta462 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1841;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1842;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta462<F: Float>(t12250: F, t20489: F, t1343: F, t820: F, t3792: F, t119: F, t20416: F, t210: F, t12291: F, t12330: F, t12335: F, t1315: F, t16341: F, t16350: F, t19915: F, t19917: F, t19933: F, t3790: F, t5235: F, t6417: F, t20356: F, t1810: F, t6347: F, t11982: F, t11984: F, t20354: F, t20355: F, t20360: F, t20361: F, t20365: F, t20366: F, t20370: F, t9457: F, t9476: F, t9484: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t20490, t20492, t20495, t20497, t20500, t20501, t20508) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1841::<F>(t12250, t20489, t1343, t820, t3792, t119, t20416, t210, t12291, t12330, t12335, t1315, t16341, t16350, t19915, t19917, t19933, t3790, t5235, t6417);
        let (t20511, t20512, t20516, t20519) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1842::<F>(t119, t20356, t210, t1810, t6347, t11982, t11984, t20354, t20355, t20360, t20361, t20365, t20366, t20370, t9457, t9476, t9484);
    (t20490, t20492, t20495, t20497, t20500, t20501, t20508, t20511, t20512, t20516, t20519)
}
