//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 670/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk670(t161: f64, t1890: f64, t2052: f64, t796: f64, t2154: f64, t2109: f64, t806: f64) -> (f64, f64, f64, f64) {
    let t5841 = t1890 * t161;
    let t5983 = t2052 * t796;
    let t6018 = t2154 * t796;
    let t6021 = t2109 * t806;
    (t5841, t5983, t6018, t6021)
}
