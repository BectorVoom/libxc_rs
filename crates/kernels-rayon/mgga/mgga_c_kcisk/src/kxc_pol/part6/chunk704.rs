//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 704/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk704(t12514: f64, t830: f64, t2885: f64, t172: f64, t849: f64, t157: f64, t2914: f64, t2922: f64, t119: f64, t814: f64, t298: f64, t831: f64) -> (f64, f64, f64, f64, f64) {
    let t12586 = t12514 * t830;
    let t12588 = 6.0_f64 * t2885 * t12586;
    let t12589 = t172 * t849;
    let t12592 = t157 * t2914;
    let t12595 = t157 * t2922;
    let t12598 = t119 * t814;
    let t12601 = 0.71233333333333333334e-1_f64 * t298 * t12598 * t831;
    (t12588, t12589, t12592, t12595, t12601)
}
