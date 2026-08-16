//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 560/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk560(t2901: f64, t2970: f64, t133: f64, t2916: f64, t793: f64, t2036: f64, t2968: f64, t2923: f64, t1133: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2971 = t2970 * t2901;
    let t2976 = t2916 * t133;
    let t2977 = t2976 * t793;
    let t2980 = t2036 * t2968;
    let t2981 = t2970 * t2923;
    let t2984 = t751 * t1133;
    (t2971, t2976, t2977, t2980, t2981, t2984)
}
