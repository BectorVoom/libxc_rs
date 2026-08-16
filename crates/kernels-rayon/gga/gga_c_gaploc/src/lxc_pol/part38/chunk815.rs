//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 815/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk815(t33399: f64, t959: f64, t13118: f64, t15362: f64, t2365: f64, t32357: f64, t6111: f64, t32436: f64, t24501: f64, t825: f64, t9438: f64, t32261: f64, t7390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43462 = t33399 * t959;
    let t43464 = t15362 * t13118;
    let t43467 = t6111 * t2365 * t32357;
    let t43470 = t6111 * t2365 * t32436;
    let t43476 = t825 * t9438 * t24501;
    let t43502 = t7390 * t2365 * t32261;
    (t43462, t43464, t43467, t43470, t43476, t43502)
}
