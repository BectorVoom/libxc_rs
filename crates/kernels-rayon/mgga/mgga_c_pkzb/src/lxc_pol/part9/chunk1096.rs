//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1096/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1096(t154: f64, t1843: f64, t276: f64, t5688: f64, t2048: f64, t5537: f64, t2050: f64, t2057: f64, t5665: f64, t735: f64, t5690: f64, t486: f64, t779: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18067 = t276 * t154 * t5688 * t1843;
    let t18071 = t276 * t154 * t2048 * t5537;
    let t18073 = t2057 * t2050;
    let t18079 = t735 * t5665;
    let t18084 = t735 * t5690;
    let t18086 = t486 * t779;
    (t18067, t18071, t18073, t18079, t18084, t18086)
}
