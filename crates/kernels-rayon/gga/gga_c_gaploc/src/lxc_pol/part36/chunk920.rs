//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 920/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk920(t39904: f64, t1063: f64, t3152: f64, t7974: f64, t41809: f64, t426: f64, t2268: f64, t535: f64, t32067: f64, t894: f64, t12820: f64, t2312: f64) -> (f64, f64, f64, f64, f64) {
    let t42874 = 0.71137516589190373998e-2_f64 * t39904;
    let t42877 = 0.28455006635676149599e-1_f64 * t1063 * t3152 * t7974;
    let t42878 = t41809 * t426;
    let t42881 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t42878;
    let t42883 = t1063 * t894 * t32067;
    let t42885 = t2312 * t12820;
    (t42874, t42877, t42881, t42883, t42885)
}
