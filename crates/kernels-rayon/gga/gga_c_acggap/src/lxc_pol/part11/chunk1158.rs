//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1158/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1158(t30817: f64, t8948: f64, t8793: f64, t4434: f64, t570: f64, t1313: f64, t30598: f64, t721: f64, t1322: f64, t7859: f64, t2041: f64, t4632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35874 = t30817 * t8948;
    let t35875 = 0.25724410870841842184e-2_f64 * t35874;
    let t35876 = t30817 * t8793;
    let t35877 = 0.37737710747524982482e-2_f64 * t35876;
    let t35879 = t570 * t4434;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    let t35887 = t2041 * t4632;
    (t35875, t35877, t35879, t35882, t35885, t35887)
}
