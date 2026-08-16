//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 759/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk759(t10281: f64, t501: f64, t1853: f64, t3432: f64, t10667: f64, t325: f64, t835: f64, t3431: f64, t723: f64, t7290: f64, t701: f64, t2610: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32100 = t10281 * t501;
    let t32112 = t3432 * t1853;
    let t32179 = t325 * t10667;
    let t32190 = t835 * t10667;
    let t32214 = t3431 * t723;
    let t32215 = t7290 * t32214;
    let t32260 = t3431 * t701;
    let t32261 = t2610 * t32260;
    (t32100, t32112, t32179, t32190, t32215, t32261)
}
