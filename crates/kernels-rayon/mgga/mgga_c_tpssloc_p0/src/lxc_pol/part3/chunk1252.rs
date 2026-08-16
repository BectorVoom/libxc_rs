//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1252/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1252(t12832: f64, t16505: f64, t3: f64, t1395: f64, t1858: f64, t5381: f64, t576: f64, t112: f64, t5363: f64, t111: f64, t1851: f64, t2319: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16506 = t12832 + t16505;
    let t16507 = t3 * t16506;
    let t16513 = 2.0_f64 * t1395 * t1858;
    let t16515 = 2.0_f64 * t576 * t5381;
    let t16521 = t5363 * t112;
    let t16524 = t1851 * t111;
    let t16535 = t576 * t2319;
    (t16506, t16507, t16513, t16515, t16521, t16524, t16535)
}
