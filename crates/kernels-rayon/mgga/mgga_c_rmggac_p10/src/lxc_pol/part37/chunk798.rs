//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 798/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk798(t14124: f64, t201: f64, t21714: f64, t236: f64, t457: f64, t551: f64, t14125: f64, t515: f64, t570: f64, t14131: f64, t9164: f64, t15411: f64, t68552: f64) -> (f64, f64, f64, f64) {
    let t74396 = t14124 * t21714 * t236 * t551 * t457 * t201;
    let t74403 = t14124 * t14125 * t515 * t570 * t457 * t201;
    let t74406 = t14131 * t14125 * t9164;
    let t74408 = t68552 * t15411;
    (t74396, t74403, t74406, t74408)
}
