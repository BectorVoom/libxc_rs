//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1308/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1308(t29895: f64, t8223: f64, t26129: f64, t8129: f64, t1453: f64, t29907: f64, t659: f64, t8138: f64, t4067: f64, t29900: f64, t8226: f64, t1444: f64, t666: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30147 = t29895 * t8223;
    let t30149 = t8129 * t26129;
    let t30152 = t29907 * t1453;
    let t30155 = t1453 * t659;
    let t30156 = t8138 * t30155;
    let t30159 = t8129 * t4067;
    let t30162 = t29900 * t8226;
    let t30164 = t1444 * t666;
    (t30147, t30149, t30152, t30155, t30156, t30159, t30162, t30164)
}
