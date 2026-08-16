//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2237/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2237(t26866: f64, t5436: f64, t17225: f64, t7624: f64, t17381: f64, t17456: f64, t17552: f64, t17674: f64, t17679: f64, t17684: f64, t26852: f64, t26867: f64, t29097: f64, t29100: f64, t3631: f64, t5270: f64, t5299: f64, t97149: f64, t97218: f64, t97250: f64, t97261: f64) -> f64 {
    let t104888 = t5436 * t26866;
    let t104894 = t7624 * t17225;
    let t104900 = -0.17149607247227894789e-2_f64 * t97149 * t17456 + 0.85748036236139473944e-3_f64 * t97261 * t17381 - 0.28582678745379824648e-3_f64 * t26867 * t17674 - 0.57165357490759649296e-3_f64 * t29097 * t17679 + 0.28582678745379824648e-3_f64 * t29100 * t17684 - 0.57165357490759649296e-3_f64 * t104888 * t3631 + 0.57165357490759649296e-3_f64 * t97218 + 0.28582678745379824648e-2_f64 * t7624 * t17552 - 0.76220476654346199061e-3_f64 * t104894 - 0.11433071498151929859e-2_f64 * t26852 * t5270 + 0.57165357490759649296e-3_f64 * t97250 * t5299;
    t104900
}
