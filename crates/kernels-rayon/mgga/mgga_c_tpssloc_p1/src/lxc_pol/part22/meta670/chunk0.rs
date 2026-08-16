//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2225/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2225(t17579: f64, t225: f64, t18048: f64, t17826: f64, t2960: f64, t10236: f64, t17686: f64, t43070: f64, t10254: f64, t17635: f64, t17691: f64, t135: f64, t17843: f64, t973: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t61058 = t17579 * t225;
    let t61061 = t18048 * t225;
    let t61074 = t2960 * t17826;
    let t61082 = t10236 * t17686;
    let t61086 = t43070 * t17686;
    let t61094 = t10254 * t17635;
    let t61103 = t10254 * t17691;
    let t61172 = t973 * t135 * t17843;
    (t61058, t61061, t61074, t61082, t61086, t61094, t61103, t61172)
}
