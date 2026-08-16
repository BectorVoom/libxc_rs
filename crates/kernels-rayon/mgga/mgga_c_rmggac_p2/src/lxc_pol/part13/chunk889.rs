//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 889/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk889(t39705: f64, t7206: f64, t7255: f64, t8422: f64, t2289: f64, t35384: f64, t1986: f64, t5142: f64, t675: f64, t7944: f64, t1971: f64, t27326: f64, t3351: f64, t7262: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39706 = t39705 * t7206;
    let t39709 = t7255 * t8422;
    let t39711 = t35384 * t2289;
    let t39715 = t675 * t1986 * t5142;
    let t39717 = t7944 * t2289;
    let t39721 = t3351 * t1971 * t7262 * t27326;
    (t39706, t39709, t39711, t39715, t39717, t39721)
}
