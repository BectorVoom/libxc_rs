//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 212/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk212(t879: f64, t880: f64, t45: f64, t68: f64, t93: f64, t98: f64, t867: f64, t869: f64, t874: f64, t877: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t881 = t879 * t880;
    let t884 = t45 * t68;
    let t889 = t45 * t93;
    let t890 = t98 * t98;
    let t891 = 1.0_f64 / t890;
    let t896 = -0.86308333333333333334e0_f64 * t867 - 0.301925e0_f64 * t869 - 0.5501625e-1_f64 * t874 - 0.82785e-1_f64 * t877;
    (t881, t884, t889, t890, t891, t896)
}
