//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1302/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1302(t29900: f64, t8139: f64, t64: f64, t9365: f64, t2332: f64, t8129: f64, t38: f64, t96: f64, t666: f64, t659: f64, t8138: f64, t2358: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t29901 = t29900 * t8139;
    let t29903 = t64 * t9365;
    let t29904 = t8129 * t2332;
    let t29907 = t38 * t96;
    let t29908 = t29907 * t666;
    let t29911 = t666 * t659;
    let t29912 = t8138 * t29911;
    let t29915 = t8129 * t2358;
    (t29901, t29903, t29904, t29907, t29908, t29911, t29912, t29915)
}
