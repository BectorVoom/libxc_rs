//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 734/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk734(t2030: f64, t5728: f64, t1854: f64, t659: f64, t5519: f64, t1898: f64, t1897: f64, t224: f64, t212: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5729 = t5728 * t2030;
    let t5734 = t659 * t1854;
    let t5745 = 0.55403703703703703703e-1_f64 * t5519;
    let t5758 = 0.28842592592592592592e-1_f64 * t5519;
    let t5771 = t659 * t1898;
    let t5775 = 1.0_f64 / t1897 / t224;
    let t5776 = t212 * t5775;
    (t5729, t5734, t5745, t5758, t5771, t5775, t5776)
}
