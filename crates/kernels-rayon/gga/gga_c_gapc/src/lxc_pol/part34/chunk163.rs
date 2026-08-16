//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 163/1427 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk163(t436: f64, t568: f64, t120: f64, t174: f64, t344: f64, t366: f64, t371: f64, t384: f64, t385: f64, t426: f64, t434: f64, t459: f64, t466: f64, t469: f64, t473: f64, t477: f64, t508: f64, t518: f64, t523: f64, t526: f64) -> (f64, f64) {
    let t569 = t436 * t568;
    let t572 = t344 + t366 - t371 - t384 - 0.17379648562707520765e-2_f64 * t385 * t174 + 0.10427789137624512459e-2_f64 * t426 * t174 - 0.10427789137624512459e-2_f64 * t434 * t459 - 0.3475929712541504153e-4_f64 * t466 * t469 + 0.61802030288987943842e-4_f64 * t473 * t477 + 0.10427789137624512459e-2_f64 * t120 * t508 + 0.50690641641230268898e-4_f64 * t518 * t523 - 0.10427789137624512459e-2_f64 * t526 * t569;
    (t569, t572)
}
