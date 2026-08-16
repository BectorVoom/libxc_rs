//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 554/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk554(t2855: f64, t922: f64, t1021: f64, t1020: f64, t2820: f64, t304: f64, t86: f64) -> (f64, f64, f64, f64) {
    let t2856 = t2855 * t922;
    let t2857 = t1021 * t2856;
    let t2858 = t1020 * t2857;
    let t2861 = t86 * t2820 * t304;
    (t2856, t2857, t2858, t2861)
}
