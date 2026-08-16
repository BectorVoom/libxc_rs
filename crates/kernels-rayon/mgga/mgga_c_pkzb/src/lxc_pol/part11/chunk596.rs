//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 596/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk596(t3265: f64, t945: f64, t2393: f64, t3258: f64, t2970: f64, t3207: f64, t1245: f64, t914: f64) -> (f64, f64, f64, f64) {
    let t3266 = t3265 * t945;
    let t3269 = t2393 * t3258;
    let t3270 = t2970 * t3207;
    let t3273 = t914 * t1245;
    (t3266, t3269, t3270, t3273)
}
