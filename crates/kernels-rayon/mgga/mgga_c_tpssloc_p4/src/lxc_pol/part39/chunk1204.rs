//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1204/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1204(t3377: f64, t4861: f64, t14722: f64, t14704: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11444: f64, t14702: f64, t14708: f64, t14720: f64, t14728: f64, t14733: f64, t14738: f64, t14742: f64, t14746: f64, t14751: f64, t14755: f64) -> (f64, f64) {
    let t15185 = t4861 * t3377;
    let t15194 = 0.2283111111111111111e-1_f64 * t14722;
    let t15195 = 0.11415555555555555555e-1_f64 * t14704;
    let t15204 = -t11444 + 0.1522074074074074074e-1_f64 * t11137 + 0.38051851851851851851e-2_f64 * t11139 - 0.11415555555555555555e-1_f64 * t11141 - 0.57077777777777777777e-2_f64 * t11143 + 0.76103703703703703702e-2_f64 * t14702 + 0.76103703703703703701e-2_f64 * t14720 - t15194 - t15195 + 0.19025925925925925925e-1_f64 * t14728 - 0.68493333333333333331e-1_f64 * t14733 - 0.2283111111111111111e-1_f64 * t14738 - 0.11415555555555555555e-1_f64 * t14742 + 0.10274e0_f64 * t14746 + 0.68493333333333333332e-1_f64 * t14751 + 0.34246666666666666666e-1_f64 * t14755 + 0.17123333333333333333e-1_f64 * t14708;
    (t15185, t15204)
}
