//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1174/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1174(t1210: f64, t1734: f64, t1409: f64, t2132: f64, t2136: f64, t210: f64, t7998: f64, t1193: f64, t8020: f64, t52: f64, t8027: f64, t461: f64, t7573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27642 = t1210 * t1734;
    let t27650 = t2132 * t1409;
    let t27651 = t27650 * t2136;
    let t27674 = t7998 * t210;
    let t27677 = t8020 * t1193;
    let t27680 = t8027 * t52;
    let t27681 = t27680 * t2136;
    let t27683 = t7573 * t461;
    (t27642, t27651, t27674, t27677, t27681, t27683)
}
