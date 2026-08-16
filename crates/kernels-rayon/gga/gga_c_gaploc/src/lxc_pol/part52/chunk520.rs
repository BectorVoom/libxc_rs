//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 520/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk520(t10023: f64, t10024: f64, t3295: f64, t7354: f64, t2684: f64, t2365: f64, t7069: f64, t7390: f64, t2440: f64, t988: f64, t2268: f64, t2756: f64, t894: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10026 = 0.89376224879626066674e-1_f64 * t10023 * t10024;
    let t10029 = t7354 * t3295;
    let t10030 = t2684 * t10029;
    let t10040 = t2365 * t7069;
    let t10042 = 0.29792074959875355558e-1_f64 * t7390 * t10040;
    let t10113 = t2440 * t988;
    let t10115 = 0.28455006635676149599e-1_f64 * t2268 * t10113;
    let t10116 = t894 * t2756;
    (t10026, t10030, t10040, t10042, t10115, t10116)
}
