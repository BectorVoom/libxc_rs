//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2260/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2260(t12984: f64, t2379: f64, t46799: f64, t686: f64, t133: f64, t1484: f64, t41214: f64, t6600: f64, t12998: f64, t46766: f64, t776: f64, t12971: f64, t12988: f64, t213: f64, t221: f64, t2553: f64, t41203: f64, t41205: f64, t4127: f64, t46788: f64, t46790: f64, t46794: f64, t46796: f64) -> f64 {
    let t46802 = t46799 * t686 * t12984 * t2379;
    let t46806 = t41214 * t133 * t6600 * t1484;
    let t46819 = t12998 * t686 * t46766 * t776;
    let t46821 = -0.75e-2_f64 * t41203 - 0.34999999999999999998e-1_f64 * t41205 + 0.11666666666666666666e-1_f64 * t46788 + 0.56172839506172839502e-1_f64 * t46790 + t46794 + 0.47499999999999999998e-1_f64 * t46796 + 0.29999999999999999998e-1_f64 * t46802 + 0.27777777777777777777e-3_f64 * t46806 + 0.14999999999999999999e-1_f64 * t4127 * t221 * t213 * t12971 * t776 + 0.14999999999999999999e-1_f64 * t4127 * t221 * t12988 * t2553 - 0.14999999999999999999e-1_f64 * t46819;
    t46821
}
