//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1186/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1186(t3: f64, t5398: f64, t1933: f64, t1618: f64, t1622: f64, t1937: f64, t23447: f64, t23537: f64, t23541: f64, t25577: f64, t25580: f64, t25598: f64, t25616: f64, t25618: f64, t25625: f64, t25629: f64, t25645: f64, t5857: f64, t5861: f64, t5869: f64, t5875: f64, t5880: f64, t6755: f64, t6765: f64, t7583: f64) -> (f64, f64, f64) {
    let t28525 = t3 * t5398;
    let t28526 = t1933 * t28525;
    let t28550 = 0.10093189023535097714e-3_f64 * t28526 * t1937 - 0.20186378047070195428e-3_f64 * t25645 * t7583 + t25598 / 432.0_f64 + t25577 * t1618 / 768.0_f64 + t25580 * t1622 / 1152.0_f64 + t6755 * t5869 / 1536.0_f64 + t23537 * t5875 / 768.0_f64 - t23541 * t5880 / 1536.0_f64 + t6765 * t5857 / 2304.0_f64 + 5.0_f64 / 6912.0_f64 * t6765 * t5861 - t23447 + t25616 / 1728.0_f64 + t25618 / 1152.0_f64 + t25625 / 1152.0_f64 + 0.20186378047070195428e-3_f64 * t25629;
    (t28525, t28526, t28550)
}
