//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 356/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk356(t2949: f64, t701: f64, t1901: f64, t550: f64, t1843: f64, t1022: f64, t835: f64) -> (f64, f64, f64, f64, f64) {
    let t2950 = t2949 * t701;
    let t2951 = t1901 * t2950;
    let t2954 = t550 * t2949;
    let t2955 = t1843 * t2954;
    let t2958 = t835 * t1022;
    (t2950, t2951, t2954, t2955, t2958)
}
