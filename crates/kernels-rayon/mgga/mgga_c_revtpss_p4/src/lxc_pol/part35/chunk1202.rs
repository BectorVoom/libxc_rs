//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1202/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1202(t102266: f64, t102293: f64, t102296: f64, t102298: f64, t102316: f64, t109512: f64, t109514: f64, t109534: f64, t109536: f64, t1903: f64, t2097: f64, t22971: f64, t23042: f64, t27837: f64, t30247: f64, t30257: f64, t6918: f64, t7295: f64, t7296: f64, t7511: f64, t8085: f64, t96284: f64) -> f64 {
    let t115152 = 0.26020884564615598386e1_f64 * t27837 * t30257 + 0.34697458558045176417e-2_f64 * t102266 - 0.38554277296572111609e-1_f64 * t109512 - 0.77108554593144223218e-1_f64 * t109514 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2097 * t23042 + 0.39512695097613069591e1_f64 * t7511 * t22971 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t8085 * t6918 - 0.10281140612419229762e0_f64 * t102293 - 0.28912093960683998208e-1_f64 * t102296 - t96284 + 0.51405703062096148812e-1_f64 * t102298 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t30247 * t1903 + 0.28912093960683998208e-1_f64 * t102316 - 0.58544643236296698113e-1_f64 * t109534 + 0.15421710918628844643e0_f64 * t109536;
    t115152
}
