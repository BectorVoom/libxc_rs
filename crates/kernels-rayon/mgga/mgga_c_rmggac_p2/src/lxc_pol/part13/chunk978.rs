//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 978/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk978(t25854: f64, t40887: f64, t2301: f64, t5245: f64, t2295: f64, t30510: f64, t40883: f64, t5259: f64, t25820: f64, t38977: f64, t27101: f64, t38980: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41458 = t25854 * t40887;
    let t41463 = t5245 * t2301;
    let t41475 = t30510 * t2295;
    let t41477 = t5259 * t40883;
    let t41488 = t25820 * t38977;
    let t41490 = t27101 * t38980;
    (t41458, t41463, t41475, t41477, t41488, t41490)
}
