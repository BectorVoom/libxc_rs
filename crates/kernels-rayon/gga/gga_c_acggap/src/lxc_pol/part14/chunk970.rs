//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 970/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk970(t34361: f64, t10146: f64, t420: f64, t576: f64, t1083: f64, t137: f64, t1511: f64, t2020: f64, t7440: f64, t8631: f64, t2318: f64, t31261: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34362 = 0.12862205435420921092e-1_f64 * t34361;
    let t34368 = t576 * t420 * t10146;
    let t34369 = t1083 * t137;
    let t34382 = t2020 * t1511;
    let t34383 = 7.0_f64 / 144.0_f64 * t34382;
    let t34390 = t7440 * t8631;
    let t34391 = 0.5603125e-1_f64 * t34390;
    let t34392 = t31261 * t2318;
    (t34362, t34368, t34369, t34383, t34391, t34392)
}
