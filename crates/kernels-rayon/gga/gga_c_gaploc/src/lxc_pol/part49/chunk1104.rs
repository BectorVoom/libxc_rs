//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1104/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1104(t47130: f64, t5241: f64, t5640: f64, t590: f64, t1890: f64, t1966: f64, t13847: f64, t825: f64, t826: f64, t12161: f64, t123: f64, t883: f64) -> (f64, f64, f64, f64) {
    let t47133 = t5640 * t5241 * t47130 * t590;
    let t47137 = t1966 * t1890 * t47130 * t590;
    let t47140 = t825 * t826 * t13847;
    let t47143 = t12161 * t123 * t883;
    (t47133, t47137, t47140, t47143)
}
