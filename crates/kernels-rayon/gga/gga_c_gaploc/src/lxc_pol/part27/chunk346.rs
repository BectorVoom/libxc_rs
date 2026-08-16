//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 346/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk346(t163: f64, t20: f64, t1476: f64, t14: f64, t72: f64, t506: f64, t397: f64, t4: f64, t78: f64, t3: f64, t97: f64, t508: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1477 = t20 * t163;
    let t1478 = t1476 * t1477;
    let t1481 = t14 * t72;
    let t1482 = t506 * t1481;
    let t1484 = t4 * t78 * t397;
    let t1487 = t3 * t97;
    let t1488 = t508 * t1487;
    (t1477, t1478, t1482, t1484, t1487, t1488)
}
