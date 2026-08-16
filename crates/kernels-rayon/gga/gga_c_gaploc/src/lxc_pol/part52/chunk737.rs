//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 737/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk737(t1423: f64, t7784: f64, t1964: f64, t9419: f64, t823: f64, t2089: f64, t40: f64, t7291: f64, t10007: f64, t10012: f64, t588: f64, t7068: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22256 = t1423 * t7784;
    let t22537 = t1964 * t9419;
    let t22542 = t823 * t9419;
    let t22623 = t40 * t2089;
    let t22624 = t22623 * t7291;
    let t22629 = t10007 * t7291;
    let t22634 = t10012 * t7291;
    let t22665 = t588 * t2089;
    let t22980 = t10007 * t7068;
    (t22256, t22537, t22542, t22623, t22624, t22629, t22634, t22665, t22980)
}
