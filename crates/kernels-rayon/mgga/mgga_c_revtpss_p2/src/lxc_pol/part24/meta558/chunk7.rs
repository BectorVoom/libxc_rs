//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1676/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1676(t88382: f64, t88396: f64, t88412: f64, t88427: f64, t915: f64, t935: f64, t1609: f64, t23547: f64, t2874: f64, t2924: f64, t78329: f64, t11385: f64, t19255: f64, t6141: f64) -> (f64, f64, f64, f64) {
    let t88432 = 1.0_f64 * t915 * (t88382 + t88396 + t88412 + t88427) * t935;
    let t88445 = 8.0_f64 * t2874 * t23547 * t1609;
    let t88448 = 0.64327917994770140268e2_f64 * t2924 * t78329 * t1609;
    let t88451 = 0.3103560775156404018e4_f64 * t11385 * t19255 * t6141;
    (t88432, t88445, t88448, t88451)
}
