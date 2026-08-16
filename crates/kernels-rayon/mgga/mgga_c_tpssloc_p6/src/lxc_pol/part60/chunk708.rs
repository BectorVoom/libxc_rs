//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 708/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk708(t6589: f64, t6597: f64, t281: f64, t6619: f64, t835: f64, t812: f64, t1891: f64, t9223: f64, t213: f64, t1895: f64, t1887: f64, t206: f64, t22715: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23121 = t6597 * t6589;
    let t23122 = t23121 * t281;
    let t23132 = t6619 * t835;
    let t23133 = t812 * t23132;
    let t23138 = t9223 * t1891;
    let t23139 = t23138 * t213;
    let t23140 = t23139 * t1895;
    let t23141 = 0.11304371706359309439e-1_f64 * t23140;
    let t23143 = t22715 * t206 * t1887;
    (t23122, t23133, t23139, t23140, t23141, t23143)
}
