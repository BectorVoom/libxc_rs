//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1078/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1078(t4844: f64, t865: f64, t2531: f64, t1425: f64, t3806: f64, t2481: f64, t4879: f64, t8600: f64, t4876: f64, t2533: f64, t4875: f64, t3810: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14863 = t4844 * t865;
    let t14865 = 6.0_f64 * t2531 * t14863;
    let t14866 = t1425 * t3806;
    let t14868 = 4.0_f64 * t2481 * t14866;
    let t14869 = t4879 * t865;
    let t14871 = 0.96491876992155210402e2_f64 * t8600 * t14869;
    let t14872 = t4876 * t865;
    let t14874 = 2.0_f64 * t2481 * t14872;
    let t14875 = t4875 * t2533;
    let t14876 = t14875 * t865;
    let t14878 = 0.16081979498692535067e2_f64 * t2531 * t14876;
    let t14879 = t3810 * t3806;
    (t14865, t14868, t14871, t14874, t14878, t14879)
}
