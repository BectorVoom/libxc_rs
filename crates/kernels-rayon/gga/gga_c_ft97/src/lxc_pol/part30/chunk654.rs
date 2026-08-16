//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 654/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk654(t10248: f64, t28516: f64, t446: f64, t25140: f64, t3886: f64, t2665: f64, t25037: f64, t10409: f64, t1486: f64, t681: f64, t7075: f64, t1882: f64, t7080: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28517 = t10248 * t28516;
    let t28518 = t446 * t28517;
    let t28520 = t25140 * t3886;
    let t28521 = t2665 * t28520;
    let t28522 = t446 * t28521;
    let t28524 = t25037 * t3886;
    let t28525 = t10409 * t28524;
    let t28526 = t446 * t28525;
    let t28529 = t1486 * t681 * t7075;
    let t28531 = t1882 * t7080;
    (t28517, t28518, t28520, t28521, t28522, t28524, t28525, t28526, t28529, t28531)
}
