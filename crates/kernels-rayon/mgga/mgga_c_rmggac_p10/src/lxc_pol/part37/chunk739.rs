//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 739/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk739(t69583: f64, t14413: f64, t638: f64, t7292: f64, t14417: f64, t2046: f64, t7297: f64, t2039: f64, t2244: f64, t270: f64, t2227: f64, t235: f64, t7190: f64) -> (f64, f64, f64, f64, f64) {
    let t71369 = 0.17347588262831798124e-3_f64 * t69583;
    let t71372 = t638 * t7292 * t14413;
    let t71373 = 0.81300399444200075504e-3_f64 * t71372;
    let t71375 = t2046 * t7297 * t14417;
    let t71376 = 0.1951603679568577289e-3_f64 * t71375;
    let t71380 = t638 * t2039 * t2244 * t270;
    let t71400 = t235 * t7190 * t2227;
    (t71369, t71373, t71376, t71380, t71400)
}
