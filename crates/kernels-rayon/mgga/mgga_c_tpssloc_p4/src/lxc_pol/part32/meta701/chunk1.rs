//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2198/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2198(t1874: f64, t96683: f64, t25992: f64, t7685: f64, t25985: f64, t28821: f64, t7000: f64, t1983: f64, t24990: f64, t26167: f64, t7687: f64, t91620: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t97831 = 4.0_f64 * t96683 * t1874;
    let t97833 = 2.0_f64 * t7685 * t25992;
    let t97835 = 6.0_f64 * t7685 * t25985;
    let t97836 = t28821 * t7000;
    let t97839 = 6.0_f64 * t1983 * t26167 * t24990;
    let t97842 = 6.0_f64 * t1983 * t91620 * t7687;
    (t97831, t97833, t97835, t97836, t97839, t97842)
}
