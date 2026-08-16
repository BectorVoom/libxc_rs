//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1576/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1576(t23133: f64, t849: f64, t1891: f64, t9223: f64, t213: f64, t1895: f64, t1887: f64, t206: f64, t22715: f64, t242: f64, t6612: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23134 = t23133 * t849;
    let t23135 = 7.0_f64 / 288.0_f64 * t23134;
    let t23138 = t9223 * t1891;
    let t23139 = t23138 * t213;
    let t23140 = t23139 * t1895;
    let t23143 = t22715 * t206 * t1887;
    let t23145 = t6612 * t242;
    let t23146 = t812 * t23145;
    (t23135, t23138, t23140, t23143, t23145, t23146)
}
