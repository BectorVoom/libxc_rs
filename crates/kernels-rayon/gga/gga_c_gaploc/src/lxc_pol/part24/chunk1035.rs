//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1035/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1035(t291: f64, t293: f64, t5343: f64, t539: f64, t835: f64, t2086: f64, t2109: f64, t2102: f64, t2154: f64, t169: f64, t4585: f64, t2683: f64, t5580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15672 = t291 / t5343 / t293;
    let t16036 = t539 * t835;
    let t16136 = t2109 * t2086;
    let t16239 = t2154 * t2102;
    let t16251 = t4585 * t169;
    let t16455 = t5580 * t2683;
    (t15672, t16036, t16136, t16239, t16251, t16455)
}
