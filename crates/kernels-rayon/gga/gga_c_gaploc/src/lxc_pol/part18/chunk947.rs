//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 947/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk947(t3381: f64, t4379: f64, t2366: f64, t2754: f64, t2365: f64, t1429: f64, t10241: f64, t447: f64) -> (f64, f64, f64, f64, f64) {
    let t10308 = t4379 * t3381;
    let t10309 = 0.14896037479937677779e-1_f64 * t10308;
    let t10310 = t2366 * t2754;
    let t10311 = t2365 * t10310;
    let t10312 = t1429 * t10311;
    let t10313 = 0.14896037479937677779e-1_f64 * t10312;
    let t10314 = t10241 * t447;
    (t10309, t10310, t10311, t10313, t10314)
}
