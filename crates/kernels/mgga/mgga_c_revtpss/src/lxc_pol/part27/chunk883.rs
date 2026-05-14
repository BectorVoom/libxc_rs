//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 883/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk883<F: Float>(t1063: F, t11744: F, t1066: F, t11160: F, t247: F, t1068: F, t11707: F, t11712: F, t11714: F, t11723: F, t11728: F, t11730: F, t11732: F, t11737: F, t11738: F, t3091: F, t3101: F, t3106: F, t3177: F, t3184: F, t348: F) -> (F,) {
    let t11745 = t1063 * t11744;
    let t11748 = t247 * t1066 * t11160;
    let t11751 = 0.7145669686344956162e-3 * t3091 * t11707 + 0.57165357490759649295e-3 * t11712 - 0.45732285992607719436e-2 * t11714 * t1068 - 0.22866142996303859718e-2 * t3106 * t3177 - 0.3811023832717309953e-2 * t3106 * t3184 + 0.28582678745379824648e-3 * t11723 + 0.47637797908966374413e-3 * t11728 + 11.0 / 108.0 * t11730 + t11732 / 54.0 + t11737 - 77.0 / 162.0 * t11738 * t348 + 0.45732285992607719436e-2 * t3106 * t3101 - 0.57165357490759649295e-3 * t11745 + 0.85748036236139473944e-3 * t1063 * t11748;
    (t11751,)
}
