//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1115/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1115(t486: f64, t931: f64, t154: f64, t2226: f64, t385: f64, t2411: f64, t67: f64, t6406: f64, t2387: f64, t5728: f64, t1478: f64, t405: f64) -> (f64, f64, f64, f64) {
    let t18989 = t486 * t931;
    let t18992 = t385 * t154 * t18989 * t2226;
    let t18994 = t67 * t2411;
    let t18997 = t385 * t154 * t18994 * t6406;
    let t19014 = t2387 * t5728;
    let t19023 = t1478 * t405;
    (t18992, t18997, t19014, t19023)
}
