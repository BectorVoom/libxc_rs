//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 726/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk726(t13629: f64, t13665: f64, t13696: f64, t13716: f64, t1052: f64, t10800: f64, t13567: f64, t13569: f64, t13573: f64, t13577: f64, t13580: f64, t13581: f64, t13584: f64, t13587: f64, t1960: f64, t2969: f64, t331: f64, t3511: f64, t748: f64) -> (f64, f64) {
    let t13718 = t13629 + t13665 + t13696 + t13716;
    let t13720 = -2.0_f64 * t1052 * t10800 + t13567 * t331 + 4.0_f64 * t13581 * t1960 - t13718 * t748 - 2.0_f64 * t2969 * t3511 - t13569 + t13573 + t13577 - t13580 - t13584 + t13587;
    (t13718, t13720)
}
