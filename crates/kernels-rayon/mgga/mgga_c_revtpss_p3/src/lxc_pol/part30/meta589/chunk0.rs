//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2048/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2048(t10073: f64, t1444: f64, t2029: f64, t25929: f64, t26041: f64, t9664: f64, t2030: f64, t47567: f64, t26069: f64, t94806: f64, t1426: f64, t94609: f64) -> (f64, f64, f64, f64, f64) {
    let t94857 = t10073 * t25929 * t2029 * t1444;
    let t94865 = 0.46263278077393568556e-2_f64 * t26041 * t9664;
    let t94867 = 0.81814717454467823679e-4_f64 * t47567 * t2030;
    let t94876 = t26069 * t94806;
    let t94878 = t94609 * t1426;
    (t94857, t94865, t94867, t94876, t94878)
}
