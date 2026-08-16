//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 884/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk884(t39904: f64, t1063: f64, t3152: f64, t7974: f64, t41809: f64, t426: f64, t2268: f64, t535: f64, t3158: f64, t8195: f64, t8199: f64, t9181: f64) -> (f64, f64, f64, f64, f64) {
    let t42874 = 0.71137516589190373998e-2_f64 * t39904;
    let t42877 = 0.28455006635676149599e-1_f64 * t1063 * t3152 * t7974;
    let t42878 = t41809 * t426;
    let t42881 = 0.28455006635676149599e-1_f64 * t2268 * t535 * t42878;
    let t42893 = 0.42682509953514224398e0_f64 * t2268 * t3158 * t8195;
    let t42896 = 0.14227503317838074799e1_f64 * t2268 * t9181 * t8199;
    (t42874, t42877, t42881, t42893, t42896)
}
