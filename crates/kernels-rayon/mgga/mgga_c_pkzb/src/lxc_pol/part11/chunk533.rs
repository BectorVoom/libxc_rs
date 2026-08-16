//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 533/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk533(t2782: f64, t684: f64, t664: f64, t1083: f64, t1901: f64, t683: f64, t1899: f64, t1833: f64, t1905: f64, t2730: f64, t2741: f64, t1088: f64, t694: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2783 = t2782 * t684;
    let t2785 = 1.0_f64 * t664 * t2783;
    let t2786 = t1083 * t1901;
    let t2787 = t2786 * t683;
    let t2789 = 0.16081979498692535067e2_f64 * t1899 * t2787;
    let t2793 = t1905 - 0.17123333333333333333e-1_f64 * t1833 - 0.17123333333333333333e-1_f64 * t2730 + 0.5137e-1_f64 * t2741;
    let t2796 = t1088 * t694;
    (t2783, t2785, t2786, t2787, t2789, t2793, t2796)
}
