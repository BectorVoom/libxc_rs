//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1063/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1063<F: Float>(t102266: F, t102293: F, t102296: F, t102298: F, t102316: F, t109512: F, t109514: F, t109534: F, t109536: F, t1903: F, t2097: F, t22971: F, t23042: F, t27837: F, t30247: F, t30257: F, t6918: F, t7295: F, t7296: F, t7511: F, t8085: F, t96284: F) -> (F,) {
    let t115152 = 0.26020884564615598386e1 * t27837 * t30257 + 0.34697458558045176417e-2 * t102266 - 0.38554277296572111609e-1 * t109512 - 0.77108554593144223218e-1 * t109514 + 0.8673628188205199462e0 * t7295 * t7296 * t2097 * t23042 + 0.39512695097613069591e1 * t7511 * t22971 + 0.26020884564615598386e1 * t7295 * t7296 * t8085 * t6918 - 0.10281140612419229762e0 * t102293 - 0.28912093960683998208e-1 * t102296 - t96284 + 0.51405703062096148812e-1 * t102298 + 0.26020884564615598386e1 * t7295 * t7296 * t30247 * t1903 + 0.28912093960683998208e-1 * t102316 - 0.58544643236296698113e-1 * t109534 + 0.15421710918628844643e0 * t109536;
    (t115152,)
}
