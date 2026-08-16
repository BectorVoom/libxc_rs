//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1074/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1074(t11798: f64, t28370: f64, t7453: f64, t19048: f64, t3284: f64, t1736: f64, t435: f64, t1084: f64, t3375: f64, t11512: f64, t3707: f64, t7375: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t33298 = t11798 * t28370 * t7453;
    let t33301 = t11798 * t3284 * t19048;
    let t33303 = t435 * t1736;
    let t33304 = t1084 * t33303;
    let t33305 = t33304 * t3375;
    let t33307 = t11512 * t3707;
    let t33309 = t1084 * t33307 * t7375;
    (t33298, t33301, t33303, t33304, t33305, t33307, t33309)
}
