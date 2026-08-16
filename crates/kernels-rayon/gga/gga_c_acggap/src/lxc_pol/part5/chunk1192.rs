//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1192/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1192(t1055: f64, t21677: f64, t345: f64, t1049: f64, t5652: f64, t21615: f64, t21620: f64, t1713: f64, t922: f64, t13761: f64, t1734: f64, t3132: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21679 = t345 * t1055 * t21677;
    let t21681 = t1049 * t5652;
    let t21684 = t345 * t1055 * t21615;
    let t21687 = t345 * t1055 * t21620;
    let t21689 = t1713 * t922;
    let t21691 = t345 * t13761 * t21689;
    let t21693 = t1734 * t922;
    let t21695 = t345 * t3132 * t21693;
    (t21679, t21681, t21684, t21687, t21689, t21691, t21693, t21695)
}
