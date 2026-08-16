//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1138/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1138(t1270: f64, t4519: f64, t2222: f64, t4435: f64, t1206: f64, t1268: f64, t4377: f64, t72: f64, t732: f64, t1173: f64, t4432: f64, t1613: f64, t2331: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12673 = t4519 * t1270;
    let t12677 = t4435 * t2222;
    let t12678 = 0.24415263074675393405e-3_f64 * t12677;
    let t12679 = t1206 * t1268;
    let t12686 = t4377 * t72;
    let t12688 = 0.36622894612013090108e-3_f64 * t12686 * t732;
    let t12689 = t1173 * t4432;
    let t12690 = 8.0_f64 * t12689;
    let t12691 = t1613 * t2331;
    (t12673, t12678, t12679, t12688, t12690, t12691)
}
