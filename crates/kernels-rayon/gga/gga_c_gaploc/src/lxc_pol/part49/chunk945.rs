//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 945/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk945(t41965: f64, t6717: f64, t6914: f64, t10532: f64, t10533: f64, t10557: f64, t30936: f64, t1: f64, t2392: f64, t3338: f64, t544: f64, t594: f64) -> (f64, f64, f64, f64) {
    let t42242 = 0.62115540045351614476e2_f64 * t6914 * t6717 * t41965;
    let t42245 = 0.27606906686822939767e2_f64 * t10532 * t10533 * t41965;
    let t42250 = 0.17875244975925213335e2_f64 * t10557 * t30936;
    let t42254 = t544 * t594 * t3338 * t1 * t2392;
    (t42242, t42245, t42250, t42254)
}
