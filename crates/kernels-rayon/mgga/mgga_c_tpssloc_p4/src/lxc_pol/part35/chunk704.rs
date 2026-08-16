//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 704/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk704(t3: f64, t6470: f64, t1401: f64, t1458: f64, t3941: f64, t5371: f64, t5456: f64, t5493: f64, t577: f64, t625: f64, t107: f64, t63: f64, t656: f64) -> (f64, f64, f64, f64, f64) {
    let t6471 = t3 * t6470;
    let t6483 = 0.45e1_f64 * t6470 * t577 + 27.0_f64 * t5371 * t1458 + 27.0_f64 * t3941 * t5456 + 0.135e2_f64 * t1401 * t5493;
    let t6503 = 8.0_f64 / 3.0_f64 * t625;
    let t6528 = t625 * t107;
    let t6529 = t6528 / 3.0_f64;
    let t6530 = t63 * t656;
    (t6471, t6483, t6503, t6529, t6530)
}
