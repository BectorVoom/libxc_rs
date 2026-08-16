//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2524/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2524(t51483: f64, t10069: f64, t14588: f64, t10518: f64, t14606: f64, t10073: f64, t14504: f64, t14575: f64, t2435: f64, t14568: f64, t1568: f64, t4503: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t51484 = 0.34697458558045176417e-2_f64 * t51483;
    let t51507 = t10069 * t14588;
    let t51512 = t14606 * t10518;
    let t51513 = 0.39029762157531132076e-1_f64 * t51512;
    let t51521 = t10073 * t14504;
    let t51522 = 0.19514881078765566038e-2_f64 * t51521;
    let t51537 = t2435 * t14575;
    let t51538 = 0.21951497276451705329e-1_f64 * t51537;
    let t51546 = t14568 * t10518;
    let t51547 = 0.39029762157531132076e-1_f64 * t51546;
    let t51548 = t4503 * t1568;
    (t51484, t51507, t51513, t51522, t51538, t51547, t51548)
}
