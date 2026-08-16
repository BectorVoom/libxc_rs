//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 884/1228 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk884(t4509: f64, t984: f64, t2770: f64, t343: f64, t2775: f64, t2769: f64, t40: f64, t698: f64, t986: f64, t973: f64, t241: f64, t625: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10235 = t4509 * t984;
    let t10236 = t343 * t2770;
    let t10254 = t343 * t2775;
    let t10276 = t2769 * t40;
    let t10277 = 1.0_f64 / t10276;
    let t10286 = t698 * t986;
    let t10287 = t973 * t10286;
    let t10292 = t625 * t241;
    (t10235, t10236, t10254, t10277, t10287, t10292)
}
