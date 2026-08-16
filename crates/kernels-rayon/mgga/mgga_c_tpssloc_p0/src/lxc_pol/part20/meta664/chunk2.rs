//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2487/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2487(t43835: f64, t43837: f64, t43839: f64, t43855: f64, t43857: f64, t43859: f64, t43861: f64, t43863: f64, t50881: f64, t50886: f64, t50897: f64, t50900: f64) -> f64 {
    let t50902 = 0.198684e1_f64 * t50881 - 0.82785e-1_f64 * t50886 + 0.11038e0_f64 * t43835 - 0.33114e0_f64 * t43837 - 0.5519e-1_f64 * t43839 - 0.91983333333333333335e-1_f64 * t43855 - 0.24528888888888888889e-1_f64 * t43857 - 0.73586666666666666668e0_f64 * t43859 + 0.27595e0_f64 * t43861 + 0.55190000000000000001e0_f64 * t43863 - 0.20128333333333333333e0_f64 * t50897 - 0.72462e1_f64 * t50900;
    t50902
}
