//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 823/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk823(t41660: f64, t10495: f64, t1424: f64, t2299: f64, t544: f64, t35180: f64, t9562: f64, t10256: f64, t30830: f64, t913: f64, t2482: f64, t3358: f64, t9263: f64) -> (f64, f64, f64, f64, f64) {
    let t41661 = 0.46011511144704899612e1_f64 * t41660;
    let t41664 = t544 * t2299 * t10495 * t1424;
    let t41666 = t35180 * t9562;
    let t41667 = 0.20854452471912748891e0_f64 * t41666;
    let t41669 = t30830 * t913 * t10256;
    let t41670 = 0.59584149919750711116e-1_f64 * t41669;
    let t41672 = t9263 * t3358 * t2482;
    (t41661, t41664, t41667, t41670, t41672)
}
