//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 962/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk962(t20489: f64, t2360: f64, t2347: f64, t21204: f64, t701: f64, t9483: f64, t173: f64, t21186: f64, t18037: f64, t3799: f64, t13598: f64, t21196: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t79697 = t2360 * t20489;
    let t79702 = t2347 * t20489;
    let t79714 = t701 * t9483 * t21204;
    let t79757 = t701 * t173 * t21186;
    let t79759 = t3799 * t18037;
    let t79782 = t701 * t13598 * t21196;
    (t79697, t79702, t79714, t79757, t79759, t79782)
}
