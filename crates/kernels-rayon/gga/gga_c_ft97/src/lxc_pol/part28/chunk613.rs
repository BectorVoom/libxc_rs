//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 613/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk613(t25872: f64, t5674: f64, t1564: f64, t23057: f64, t925: f64, t1316: f64, t3051: f64, t3052: f64, t5675: f64, t473: f64, t942: f64, t1871: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25873 = t5674 * t25872;
    let t25875 = t1564 * t23057 * t925;
    let t25876 = t5674 * t25875;
    let t25878 = t1316 * t3051;
    let t25880 = t1564 * t5675 * t3052;
    let t25881 = t25878 * t25880;
    let t25883 = t942 * t473;
    let t25885 = t1871 * t5675 * t25883;
    (t25873, t25876, t25878, t25881, t25883, t25885)
}
