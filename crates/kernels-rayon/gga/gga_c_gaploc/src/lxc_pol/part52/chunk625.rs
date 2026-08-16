//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 625/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk625(t3645: f64, t773: f64, t1: f64, t3614: f64, t106: f64, t316: f64) -> (f64, f64, f64, f64) {
    let t11760 = t773 * t3645;
    let t11763 = t3614 * t1;
    let t11764 = t11763 * t106;
    let t11765 = t11764 * t316;
    (t11760, t11763, t11764, t11765)
}
