//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1167/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1167(t34465: f64, t3714: f64, t11447: f64, t33490: f64, t11452: f64, t11522: f64, t21778: f64, t8677: f64, t11523: f64, t26226: f64, t19670: f64, t8681: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34507 = t34465 * t3714;
    let t34509 = t11447 * t33490;
    let t34510 = t34509 * t11452;
    let t34515 = t21778 * t11522 * t8677;
    let t34517 = t11523 * t26226;
    let t34520 = t19670 * t11522 * t8681;
    (t34507, t34509, t34510, t34515, t34517, t34520)
}
