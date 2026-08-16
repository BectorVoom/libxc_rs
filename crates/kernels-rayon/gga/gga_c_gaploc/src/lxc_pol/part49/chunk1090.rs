//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1090/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1090(t11977: f64, t2268: f64, t6763: f64, t1063: f64, t6750: f64, t42857: f64, t42863: f64, t42866: f64, t42867: f64, t42868: f64, t42869: f64, t42870: f64, t42871: f64, t42872: f64) -> f64 {
    let t47047 = t2268 * t11977 * t6763;
    let t47050 = t1063 * t11977 * t6750;
    let t47052 = -0.19918504644973304719e0_f64 * t47047 + t42857 + t42863 + t42866 - t42867 + t42868 + 0.85365019907028448797e-1_f64 * t47050 - t42869 - t42870 - t42871 + t42872;
    t47052
}
