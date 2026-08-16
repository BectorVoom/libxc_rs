//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2124/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2124(t4021: f64, t72: f64, t7431: f64, t1864: f64, t5389: f64, t12571: f64, t1410: f64, t27971: f64, t645: f64, t1437: f64, t7445: f64, t27975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96422 = t72 * t7431 * t4021;
    let t96425 = t1864 * t5389;
    let t96443 = t12571 * t1410;
    let t96458 = t72 * t27971 * t645;
    let t96461 = t7445 * t1437;
    let t96466 = t72 * t27975 * t645;
    (t96422, t96425, t96443, t96458, t96461, t96466)
}
