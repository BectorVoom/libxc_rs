//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1028/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1028(t12448: f64, t12463: f64, t1169: f64, t1159: f64, t3475: f64, t426: f64, t3478: f64, t434: f64, t12430: f64, t1179: f64, t3488: f64, t1175: f64, t3520: f64) -> (f64, f64, f64, f64, f64) {
    let t12464 = t12448 + t12463;
    let t12465 = t12464 * t1169;
    let t12469 = 1.0_f64 / t3475 / t1159;
    let t12470 = t426 * t12469;
    let t12472 = 1.0_f64 / t3478 / t434;
    let t12473 = t12430 * t12472;
    let t12476 = t3488 * t1179;
    let t12481 = t1175 * t3520;
    (t12465, t12470, t12473, t12476, t12481)
}
