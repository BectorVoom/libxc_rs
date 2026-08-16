//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2237/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2237<F: Float>(t26866: F, t5436: F, t17225: F, t7624: F, t17381: F, t17456: F, t17552: F, t17674: F, t17679: F, t17684: F, t26852: F, t26867: F, t29097: F, t29100: F, t3631: F, t5270: F, t5299: F, t97149: F, t97218: F, t97250: F, t97261: F) -> F {
    let t104888 = t5436 * t26866;
    let t104894 = t7624 * t17225;
    let t104900 = -F::cast_from(0.17149607247227894789e-2_f64) * t97149 * t17456 + F::cast_from(0.85748036236139473944e-3_f64) * t97261 * t17381 - F::cast_from(0.28582678745379824648e-3_f64) * t26867 * t17674 - F::cast_from(0.57165357490759649296e-3_f64) * t29097 * t17679 + F::cast_from(0.28582678745379824648e-3_f64) * t29100 * t17684 - F::cast_from(0.57165357490759649296e-3_f64) * t104888 * t3631 + F::cast_from(0.57165357490759649296e-3_f64) * t97218 + F::cast_from(0.28582678745379824648e-2_f64) * t7624 * t17552 - F::cast_from(0.76220476654346199061e-3_f64) * t104894 - F::cast_from(0.11433071498151929859e-2_f64) * t26852 * t5270 + F::cast_from(0.57165357490759649296e-3_f64) * t97250 * t5299;
    t104900
}
