//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 822/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk822(t13065: f64, t2013: f64, t43710: f64, t825: f64, t969: f64, t13072: f64, t32969: f64, t10867: f64, t41511: f64, t25070: f64, t7427: f64, t9438: f64) -> (f64, f64, f64, f64, f64) {
    let t43919 = t2013 * t13065;
    let t43922 = t825 * t969 * t43710;
    let t43925 = t32969 * t13072;
    let t43927 = t10867 * t41511;
    let t43930 = t7427 * t9438 * t25070;
    (t43919, t43922, t43925, t43927, t43930)
}
