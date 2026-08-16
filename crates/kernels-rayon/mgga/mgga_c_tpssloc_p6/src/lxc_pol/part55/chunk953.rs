//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 953/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk953(t1268: f64, t26135: f64, t12725: f64, t1874: f64, t510: f64, t652: f64, t7000: f64, t7685: f64, t6876: f64, t7688: f64, t6999: f64, t7753: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26137 = 2.0_f64 * t1268 * t26135;
    let t26141 = 2.0_f64 * t12725 * t1874;
    let t26142 = t510 * t26135;
    let t26144 = 2.0_f64 * t652 * t26142;
    let t26145 = t7685 * t7000;
    let t26147 = 3.0_f64 * t6876 * t7688;
    let t26149 = t7753 * t6999;
    (t26137, t26141, t26142, t26144, t26145, t26147, t26149)
}
