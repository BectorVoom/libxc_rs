//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1007/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1007(t22740: f64, t3792: f64, t22897: f64, t1992: f64, t22751: f64, t6892: f64, t6883: f64, t6908: f64, t3719: f64, t6890: f64, t6889: f64, t6888: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22898 = t22740 * t3792;
    let t22899 = t22897 * t22898;
    let t22900 = t1992 * t22899;
    let t22907 = t22751 * t6892;
    let t22909 = t6883 * t6908;
    let t22916 = t6890 * t3719;
    let t22917 = t6889 * t22916;
    let t22918 = t6888 * t22917;
    (t22898, t22899, t22900, t22907, t22909, t22916, t22917, t22918)
}
