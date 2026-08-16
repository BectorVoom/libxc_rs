//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1120/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1120(t2960: f64, t6338: f64, t934: f64, t18653: f64, t2970: f64, t26: f64, t18645: f64, t18661: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t18828: f64, t18830: f64, t18833: f64, t18835: f64) -> (f64, f64, f64) {
    let t18889 = t2960 * t6338;
    let t18890 = t18889 * t934;
    let t18903 = t2970 * t18653;
    let t18904 = t26 * t18903;
    let t18906 = 0.1898925e1_f64 * t18835 + 0.142419375e1_f64 * t18828 - 0.1898925e1_f64 * t18830 - 0.9494625e0_f64 * t18833 - 0.19931111111111111111e0_f64 * t18674 + 0.59793333333333333334e0_f64 * t18679 + 0.66437037037037037037e-1_f64 * t18645 - 0.19931111111111111111e0_f64 * t18661 + 0.99655555555555555557e-1_f64 * t18669 - 0.29896666666666666667e0_f64 * t18683 + 0.16431333333333333333e0_f64 * t18904;
    (t18890, t18904, t18906)
}
