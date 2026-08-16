//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1873/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1873(t1992: f64, t550: f64, t57499: f64, t6976: f64, t22704: f64, t22705: f64, t28163: f64, t57618: f64, t22881: f64, t6347: f64, t6637: f64, t6888: f64) -> (f64, f64, f64, f64) {
    let t97023 = t1992 * t6976 * t57499 * t550;
    let t97026 = t22704 * t22705 * t28163;
    let t97030 = t1992 * t6976 * t57618 * t550;
    let t97036 = t6888 * t6637 * t22881 * t6347;
    (t97023, t97026, t97030, t97036)
}
