//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 891/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk891(t533: f64, t8488: f64, t1390: f64, t1983: f64, t2018: f64) -> (f64, f64, f64, f64) {
    let t8489 = t533 * t8488;
    let t8490 = t8489 * t1390;
    let t8491 = t1983 * t8490;
    let t8492 = t2018 * t2018;
    (t8489, t8490, t8491, t8492)
}
