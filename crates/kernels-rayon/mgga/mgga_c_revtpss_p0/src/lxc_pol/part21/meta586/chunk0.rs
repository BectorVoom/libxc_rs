//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2303/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2303(t1501: f64, t670: f64, t14613: f64, t162: f64, t1553: f64, t73: f64, t2723: f64, t4423: f64, t1544: f64, t890: f64, t1651: f64, t3268: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18227 = t1501 * t670;
    let t18259 = t14613 * t162;
    let t18592 = t1553 * t73;
    let t18632 = t2723 * t4423;
    let t18875 = t1544 * t890;
    let t19428 = t3268 * t1651;
    (t18227, t18259, t18592, t18632, t18875, t19428)
}
