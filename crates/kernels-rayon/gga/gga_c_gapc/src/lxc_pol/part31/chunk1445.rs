//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1445/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1445(t2845: f64, t36290: f64, t36295: f64, t36303: f64, t36304: f64, t36312: f64, t36314: f64, t36318: f64, t36326: f64, t36455: f64, t36462: f64, t38695: f64, t38699: f64, t38702: f64, t38705: f64, t38706: f64, t38708: f64, t38710: f64, t38834: f64, t38838: f64, t3916: f64) -> f64 {
    let t38866 = t2845 * t3916 + t36290 + t36295 + t36303 - t36304 + t36312 + t36314 + t36318 + t36326 - t36455 + t36462 + t38695 - t38699 + t38702 - t38705 - t38706 - t38708 + t38710 - t38834 + t38838;
    t38866
}
