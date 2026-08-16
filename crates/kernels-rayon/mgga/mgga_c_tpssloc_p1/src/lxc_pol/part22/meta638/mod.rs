//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta638 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2176;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2177;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta638(t19572: f64, t67: f64, t758: f64, t2221: f64, t6328: f64, t2225: f64, t17: f64, t2516: f64, t6320: f64, t750: f64, t19644: f64, t225: f64, t20038: f64, t212: f64, t6330: f64, t2586: f64, t40353: f64, t6347: f64, t12225: f64, t118: f64, t19631: f64, t3739: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t56374, t56390, t56394, t56398, t56400, t56422) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2176(t19572, t67, t758, t2221, t6328, t2225, t17, t2516, t6320, t750, t19644, t225);
        let (t56434, t56463, t56465, t56467, t56469, t56482) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2177(t20038, t225, t212, t6330, t2586, t40353, t6347, t12225, t118, t19631, t3739, t794);
    (t56374, t56390, t56394, t56398, t56400, t56422, t56434, t56463, t56465, t56467, t56469, t56482)
}
