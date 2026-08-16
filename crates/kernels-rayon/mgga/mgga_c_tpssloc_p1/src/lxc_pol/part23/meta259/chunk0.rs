//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 920/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk920(t118: f64, t6330: f64, t794: f64, t12202: f64, t6347: f64, t3739: f64, t12211: f64, t6353: f64, t213: f64, t3726: f64, t6358: f64, t1814: f64, t5343: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19767 = t118 * t794 * t6330;
    let t19768 = t12202 * t19767;
    let t19775 = t118 * t794 * t6347;
    let t19776 = t3739 * t19775;
    let t19779 = t12211 * t6353;
    let t19781 = t213 * t6330;
    let t19791 = t3726 * t6358;
    let t19810 = t1814 * t5343;
    (t19767, t19768, t19775, t19776, t19779, t19781, t19791, t19810)
}
