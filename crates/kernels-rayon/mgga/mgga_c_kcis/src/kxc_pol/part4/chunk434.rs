//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 434/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk434(t1670: f64, t932: f64, t939: f64, t1662: f64, t945: f64, t26: f64, t1664: f64, t937: f64, t944: f64, t950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1671 = t932 * t1670;
    let t1674 = t939 * t1670;
    let t1676 = t945 * t1662;
    let t1677 = t26 * t1676;
    let t1679 = 0.1898925e1_f64 * t1671 - t937 - 0.29896666666666666667e0_f64 * t1664 + 0.3071625e0_f64 * t1674 - t944 - 0.82156666666666666667e-1_f64 * t1677;
    let t1680 = t1679 * t950;
    (t1671, t1674, t1676, t1677, t1679, t1680)
}
