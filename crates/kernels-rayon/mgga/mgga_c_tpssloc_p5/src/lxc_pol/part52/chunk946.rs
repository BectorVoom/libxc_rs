//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 946/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk946(t23122: f64, t23124: f64, t2617: f64, t6620: f64, t6619: f64, t835: f64, t812: f64, t849: f64, t1891: f64, t9223: f64, t213: f64, t1895: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23125 = t23122 * t23124;
    let t23127 = t2617 * t6620;
    let t23132 = t6619 * t835;
    let t23133 = t812 * t23132;
    let t23134 = t23133 * t849;
    let t23138 = t9223 * t1891;
    let t23139 = t23138 * t213;
    let t23140 = t23139 * t1895;
    (t23125, t23127, t23133, t23134, t23139, t23140)
}
