//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3386/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3386(t5019: f64, t11591: f64, t6227: f64, t6219: f64, t19077: f64, t914: f64, t936: f64, t15235: f64, t4724: f64, t981: f64, t19255: f64, t2875: f64, t41588: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t63601 = t5019 * t5019;
    let t63607 = 0.17315859105681463759e2_f64 * t11591 * t6227;
    let t63609 = 0.11696447245269292414e1_f64 * t11591 * t6219;
    let t63610 = t19077 * t914;
    let t63612 = 2.0_f64 * t63610 * t936;
    let t63615 = 0.23392894490538584828e1_f64 * t981 * t4724 * t15235;
    let t63618 = 0.62071215503128080361e4_f64 * t41588 * t19255 * t2875;
    (t63601, t63607, t63609, t63612, t63615, t63618)
}
