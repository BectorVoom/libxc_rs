//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 489/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk489(t116: f64, t206: f64, t212: f64, t2586: f64, t2562: f64, t2564: f64, t2569: f64, t2571: f64, t2573: f64, t2579: f64, t2582: f64, t787: f64) -> (f64, f64, f64) {
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3_f64 * t2586 * t2588;
    let t2591 = t2562 + 0.77777777777777777775e-2_f64 * t2564 + t2569 + 0.49999999999999999998e-2_f64 * t2571 * t2573 + 0.16666666666666666666e-2_f64 * t2579 - 0.16666666666666666666e-2_f64 * t787 * t2582 - t2590;
    (t2587, t2588, t2591)
}
