//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 977/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk977(t10639: f64, t10674: f64, t158: f64, t1054: f64, t3466: f64, t5418: f64, t2678: f64, t3487: f64, t10627: f64, t183: f64, t1044: f64, t3410: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10675 = t10639 + t10674;
    let t10676 = t10675 * t158;
    let t10685 = t3466 * t1054;
    let t10686 = t5418 * t10685;
    let t10689 = t2678 * t3487;
    let t10692 = t183 * t10627;
    let t10696 = t1044 * t3410;
    (t10675, t10676, t10685, t10686, t10689, t10692, t10696)
}
