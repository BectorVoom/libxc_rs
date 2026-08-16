//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1640/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1640(t10022: f64, t14230: f64, t2782: f64, t1892: f64, t4086: f64, t786: f64) -> (f64, f64, f64, f64) {
    let t14231 = t10022 * t14230;
    let t14233 = 0.21951497276451705328e-1_f64 * t2782 * t14231;
    let t14238 = t4086 * t1892;
    let t14239 = t786 * t14238;
    (t14231, t14233, t14238, t14239)
}
