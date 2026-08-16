//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1172/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1172(t11522: f64, t21778: f64, t8677: f64, t11523: f64, t26226: f64, t19670: f64, t8681: f64, t11526: f64, t26778: f64, t21655: f64, t26369: f64, t34419: f64, t5541: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34515 = t21778 * t11522 * t8677;
    let t34517 = t11523 * t26226;
    let t34520 = t19670 * t11522 * t8681;
    let t34522 = t11526 * t26778;
    let t34525 = t21655 * t11522 * t26369;
    let t34528 = t5541 * t34419 * t8677;
    (t34515, t34517, t34520, t34522, t34525, t34528)
}
