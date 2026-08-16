//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1970/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1970(t1714: f64, t4899: f64, t11571: f64, t11545: f64, t60: f64, t461: f64) -> (f64, f64, f64, f64) {
    let t15390 = t4899 * t1714;
    let t15391 = t15390 * t11571;
    let t15394 = t60 * t11545;
    let t15395 = t15394 * t461;
    (t15390, t15391, t15394, t15395)
}
