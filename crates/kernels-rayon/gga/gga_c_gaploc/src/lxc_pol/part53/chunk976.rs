//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 976/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk976(t1445: f64, t47187: f64, t723: f64, t813: f64, t2536: f64, t3720: f64, t2009: f64, t2021: f64, t47294: f64, t7572: f64, t7573: f64, t12252: f64, t2628: f64) -> (f64, f64, f64, f64) {
    let t47442 = 0.46011511144704899612e1_f64 * t813 * t1445 * t47187 * t723;
    let t47443 = t2536 * t3720;
    let t47445 = t2021 * t47443 * t2009;
    let t47448 = t7572 * t7573 * t47294;
    let t47450 = t12252 * t2628;
    (t47442, t47445, t47448, t47450)
}
