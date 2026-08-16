//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1039/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1039(t12946: f64, t12922: f64, t12926: f64, t12934: f64, t16618: f64, t16622: f64, t16623: f64, t16624: f64, t16629: f64, t16631: f64, t16633: f64, t16636: f64, t9726: f64, t9780: f64, t9789: f64, t9863: f64) -> (f64, f64) {
    let t16685 = 8.0_f64 * t12946;
    let t16686 = t9726 + t9863 + t9780 - t16618 + t16622 + t12922 + t12926 + t16623 - t16624 + t12934 + t16629 + t16631 + t16633 + t16636 + t16685 - t9789;
    (t16685, t16686)
}
