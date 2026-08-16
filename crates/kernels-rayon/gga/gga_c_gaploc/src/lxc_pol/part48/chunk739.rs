//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 739/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk739(t1959: f64, t2590: f64, t19531: f64, t486: f64, t169: f64, t18310: f64, t1381: f64, t2353: f64, t2967: f64, t10007: f64, t8502: f64, t10012: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23575 = t2590 * t1959;
    let t23915 = t19531 * t486;
    let t24139 = t18310 * t169;
    let t24215 = t2353 * t1381;
    let t24295 = t2967 * t1959;
    let t24501 = t10007 * t8502;
    let t24505 = t10012 * t8502;
    (t23575, t23915, t24139, t24215, t24295, t24501, t24505)
}
