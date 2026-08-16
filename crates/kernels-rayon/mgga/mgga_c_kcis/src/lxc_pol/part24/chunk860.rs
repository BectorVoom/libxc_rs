//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 860/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk860(t6338: f64, t934: f64, t6349: f64, t932: f64, t6320: f64, t9758: f64, t4625: f64, t4657: f64, t2943: f64, t18685: f64, t18645: f64, t18650: f64, t18655: f64, t18659: f64, t18661: f64, t18664: f64, t18667: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18817 = t6338 * t934;
    let t18824 = t932 * t6349;
    let t18827 = t9758 * t6320;
    let t18828 = t18827 * t934;
    let t18830 = t4657 * t4625;
    let t18832 = t2943 * t6338;
    let t18833 = t18832 * t934;
    let t18835 = t932 * t18685;
    let t18853 = 0.91722222222222222223e-3_f64 * t18645 - 0.45861111111111111112e-2_f64 * t18650 + 0.1651e-1_f64 * t18655 - 0.11006666666666666667e-1_f64 * t18659 - 0.27516666666666666667e-2_f64 * t18661 - 0.24765e-1_f64 * t18664 + 0.3302e-1_f64 * t18667 + 0.13758333333333333333e-2_f64 * t18669 - 0.27516666666666666667e-2_f64 * t18674 + 0.8255e-2_f64 * t18679 - 0.41275e-2_f64 * t18683;
    (t18817, t18824, t18828, t18830, t18833, t18835, t18853)
}
