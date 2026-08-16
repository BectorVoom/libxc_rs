//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1299/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1299(t64: f64, t9365: f64, t2199: f64, t3929: f64, t1266: f64, t8189: f64, t2196: f64, t2281: f64, t29895: f64, t8181: f64, t29900: f64, t8185: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29903 = t64 * t9365;
    let t30035 = t2199 * t3929;
    let t30038 = t1266 * t8189;
    let t30048 = 11.0_f64 / 9.0_f64 * t2281 * t2196;
    let t30049 = t29895 * t8181;
    let t30051 = t29900 * t8185;
    (t29903, t30035, t30038, t30048, t30049, t30051)
}
