//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 990/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk990<F: Float>(t11735: F, t345: F, t10345: F, t344: F, t247: F, t2858: F, t3109: F, t1063: F, t1066: F, t11160: F, t1068: F, t11707: F, t11712: F, t11714: F, t11723: F, t11728: F, t11730: F, t11732: F, t3091: F, t3101: F, t3106: F, t3177: F, t3184: F, t348: F) -> (F, F, F) {
    let t11737 = F::new(5.0) / F::new(1296.0) * t345 * t11735;
    let t11738 = t10345 * t344;
    let t11744 = t247 * t3109 * t2858;
    let t11745 = t1063 * t11744;
    let t11748 = t247 * t1066 * t11160;
    let t11751 = F::new(0.7145669686344956162e-3) * t3091 * t11707 + F::new(0.57165357490759649295e-3) * t11712 - F::new(0.45732285992607719436e-2) * t11714 * t1068 - F::new(0.22866142996303859718e-2) * t3106 * t3177 - F::new(0.3811023832717309953e-2) * t3106 * t3184 + F::new(0.28582678745379824648e-3) * t11723 + F::new(0.47637797908966374413e-3) * t11728 + F::new(11.0) / F::new(108.0) * t11730 + t11732 / F::new(54.0) + t11737 - F::new(77.0) / F::new(162.0) * t11738 * t348 + F::new(0.45732285992607719436e-2) * t3106 * t3101 - F::new(0.57165357490759649295e-3) * t11745 + F::new(0.85748036236139473944e-3) * t1063 * t11748;
    (t11744, t11748, t11751)
}
