//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1467/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1467(t2482: f64, t27: f64, t4000: f64, t1419: f64, t4086: f64, t786: f64, t555: f64, t5744: f64) -> (f64, f64, f64, f64) {
    let t10001 = t2482 * t4000 * t27;
    let t10013 = t4086 * t1419;
    let t10014 = t786 * t10013;
    let t10022 = t5744 * t555;
    (t10001, t10013, t10014, t10022)
}
