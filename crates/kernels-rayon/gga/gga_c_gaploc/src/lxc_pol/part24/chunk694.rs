//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 694/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk694(t2318: f64, t6498: f64, t2322: f64, t484: f64, t2310: f64, t423: f64, t481: f64, t4260: f64, t486: f64) -> (f64, f64, f64, f64) {
    let t6499 = t6498 * t2318;
    let t6501 = t484 * t2322;
    let t6504 = t481 * t2310 * t423;
    let t6505 = t6504 * t2318;
    let t6507 = t4260 * t486;
    (t6499, t6501, t6505, t6507)
}
