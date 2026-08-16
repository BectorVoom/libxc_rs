//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 959/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk959(t25491: f64, t7581: f64, t1466: f64, t24986: f64, t2336: f64, t34290: f64, t2252: f64, t342: f64, t7574: f64, t34305: f64, t630: f64, t1774: f64, t6343: f64, t7570: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t142503 = t7581 * t25491;
    let t142512 = t1466 * t24986;
    let t142527 = t1466 * t2336 * t34290;
    let t142537 = t342 * t2252 * t7574 / 18.0_f64;
    let t142539 = t342 * t630 * t34305;
    let t142558 = t7570 * t1774 * t6343;
    (t142503, t142512, t142527, t142537, t142539, t142558)
}
