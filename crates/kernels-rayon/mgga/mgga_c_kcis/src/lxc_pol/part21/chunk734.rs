//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 734/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk734(t7718: f64, t8047: f64, t1020: f64, t1748: f64, t2179: f64, t303: f64, t1768: f64, t7726: f64, t1774: f64, t356: f64, t2173: f64, t2175: f64, t7690: f64, t7701: f64, t7703: f64, t7717: f64, t8030: f64, t8034: f64, t8038: f64, t8042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8048 = t7718 * t8047;
    let t8049 = t1020 * t8048;
    let t8051 = t1748 * t2179;
    let t8052 = t303 * t8051;
    let t8054 = t7726 * t1768;
    let t8055 = t303 * t8054;
    let t8057 = t356 * t1774;
    let t8058 = t303 * t8057;
    let t8060 = -0.69505208333333333333e-3_f64 * t8030 * t2175 + 0.92754700520833333333e-4_f64 * t7690 * t8034 - t7701 - 0.23168402777777777778e-3_f64 * t7703 * t8038 + 0.69505208333333333333e-3_f64 * t2173 * t8042 + 0.69505208333333333333e-3_f64 * t2173 * t8034 + t7717 + 0.16581944444444444444e-2_f64 * t8049 + 0.24872916666666666666e-2_f64 * t8052 - 0.24872916666666666666e-2_f64 * t8055 + 0.16581944444444444444e-2_f64 * t8058;
    (t8048, t8049, t8051, t8052, t8054, t8055, t8057, t8058, t8060)
}
