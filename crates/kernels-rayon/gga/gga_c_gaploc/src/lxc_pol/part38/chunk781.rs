//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 781/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk781(t22984: f64, t7584: f64, t9438: f64, t12692: f64, t2013: f64, t10007: f64, t2530: f64, t825: f64, t12705: f64, t7416: f64, t10012: f64, t2684: f64) -> (f64, f64, f64, f64, f64) {
    let t41244 = t7584 * t9438 * t22984;
    let t41295 = t2013 * t12692;
    let t41299 = t825 * t9438 * t10007 * t2530;
    let t41312 = t7416 * t12705;
    let t41316 = t2684 * t9438 * t10012 * t2530;
    (t41244, t41295, t41299, t41312, t41316)
}
