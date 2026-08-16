//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 722/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk722(t550: f64, t72: f64, t245: f64, t125: f64, t1882: f64, t1873: f64, t3957: f64, t1892: f64, t213: f64) -> (f64, f64, f64, f64) {
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5674 = t125 * t1882;
    let t5681 = t3957 * t1873;
    let t5715 = t213 * t1892;
    (t5673, t5674, t5681, t5715)
}
