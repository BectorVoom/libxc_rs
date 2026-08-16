//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 826/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk826(t1882: f64, t2790: f64, t2660: f64, t2796: f64, t1879: f64, t2688: f64, t5129: f64, t587: f64, t2555: f64, t5125: f64, t197: f64, t5283: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7617 = 16.0_f64 / 45.0_f64 * t2790 * t1882;
    let t7619 = 16.0_f64 / 45.0_f64 * t2660 * t2796;
    let t7623 = 16.0_f64 / 45.0_f64 * t1879 * t2796;
    let t7663 = t5129 * t2688;
    let t7665 = 16.0_f64 / 135.0_f64 * t587 * t7663;
    let t7666 = t5125 * t2555;
    let t7668 = 32.0_f64 / 135.0_f64 * t587 * t7666;
    let t7669 = t5283 * t197;
    (t7617, t7619, t7623, t7665, t7668, t7669)
}
