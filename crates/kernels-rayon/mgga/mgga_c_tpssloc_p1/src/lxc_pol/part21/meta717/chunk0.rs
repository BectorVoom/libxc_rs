//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2558/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2558(t3070: f64, t43198: f64, t4578: f64, t4574: f64, t14192: f64, t2960: f64, t10510: f64, t4641: f64, t1020: f64, t1616: f64, t248: f64, t43216: f64) -> (f64, f64, f64, f64, f64) {
    let t50147 = t3070 * t43198 * t4578;
    let t50169 = t3070 * t43198 * t4574;
    let t50172 = t2960 * t14192;
    let t50174 = t4641 * t10510;
    let t50181 = t1020 * t248 * t43216 * t1616;
    (t50147, t50169, t50172, t50174, t50181)
}
