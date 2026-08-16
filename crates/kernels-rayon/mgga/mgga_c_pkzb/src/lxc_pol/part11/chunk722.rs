//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 722/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk722(t499: f64, t52: f64, t146: f64, t155: f64, t95: f64, t625: f64, t1975: f64, t712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5401 = 1.0_f64 / t52 / t499;
    let t5402 = t146 * t5401;
    let t5405 = 455.0_f64 / 1296.0_f64 * t5402 * t95 * t155;
    let t5417 = t625 * t625;
    let t5418 = 1.0_f64 / t5417;
    let t5490 = 1.0_f64 / t1975 / t712;
    (t5401, t5402, t5405, t5417, t5418, t5490)
}
