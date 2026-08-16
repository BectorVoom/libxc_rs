//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1379/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1379(t1873: f64, t3957: f64, t1353: f64, t1872: f64, t800: f64, t124: f64, t5591: f64) -> (f64, f64, f64) {
    let t5681 = t3957 * t1873;
    let t5686 = t800 * t1872 * t1353;
    let t5689 = t124 * t5591;
    (t5681, t5686, t5689)
}
