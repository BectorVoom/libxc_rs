//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1174/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1174(t29671: f64, t8130: f64, t8133: f64, t1881: f64, t8256: f64, t637: f64, t6895: f64, t2233: f64, t12861: f64, t1607: f64, t4314: f64, t4455: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29672 = t29671 / 8.0_f64;
    let t29673 = t8130 * t8133;
    let t29674 = t29673 / 8.0_f64;
    let t29675 = t1881 * t8256;
    let t29676 = t29675 / 8.0_f64;
    let t29677 = t6895 * t637;
    let t29678 = t2233 * t29677;
    let t29679 = t29678 / 16.0_f64;
    let t30409 = t1607 * t12861;
    let t30424 = t4455 * t4314;
    (t29672, t29674, t29676, t29679, t30409, t30424)
}
