//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 871/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk871(t13819: f64, t8466: f64, t2010: f64, t70512: f64, t8465: f64, t14363: f64, t15235: f64, t15318: f64, t14267: f64, t2339: f64, t3056: f64, t2323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t75580 = t13819 * t8466;
    let t75583 = t2010 * t8465 * t70512;
    let t75585 = t14363 * t15235;
    let t75587 = t14363 * t15318;
    let t75590 = t3056 * t14267 * t2339;
    let t75593 = t3056 * t14267 * t2323;
    (t75580, t75583, t75585, t75587, t75590, t75593)
}
