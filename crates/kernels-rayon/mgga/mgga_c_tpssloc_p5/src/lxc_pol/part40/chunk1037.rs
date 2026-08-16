//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1037/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1037(t12922: f64, t12926: f64, t12934: f64, t16612: f64, t16618: f64, t16622: f64, t16623: f64, t16624: f64, t16625: f64, t16629: f64, t16631: f64, t16633: f64, t16636: f64, t16662: f64, t193: f64, t2522: f64, t4255: f64, t4310: f64, t4314: f64, t766: f64, t776: f64, t9715: f64, t9724: f64, t9726: f64, t9780: f64, t9863: f64) -> f64 {
    let t16666 = -3.0_f64 * t16625 * t2522 * t776 + 3.0_f64 * t16662 * t193 * t766 + 12.0_f64 * t4255 * t4310 * t4314 + t12922 + t12926 + t12934 + t16612 - t16618 + t16622 + t16623 - t16624 + t16629 + t16631 + t16633 + t16636 - t9715 + t9724 + t9726 + t9780 + t9863;
    t16666
}
