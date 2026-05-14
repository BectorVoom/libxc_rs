//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 813/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk813<F: Float>(t1707: F, t29138: F, t11091: F, t11092: F, t17382: F, t17385: F, t23472: F, t23481: F, t23570: F, t29088: F, t29094: F, t29116: F, t29121: F, t29124: F, t29126: F, t11003: F, t1248: F, t28369: F) -> (F, F, F) {
    let t29139 = t1707 * t29138;
    let t29146 = -0.16557e0 * t29116 - 0.40256666666666666668e0 * t17382 - 0.5519e0 * t17385 + 0.99342e0 * t29121 + 0.19419375e1 * t29124 - t11091 - t11092 - 0.412621875e-1 * t29126 + 0.258925e1 * t29139 - 0.66228e0 * t23570 - 0.60385000000000000001e0 * t23472 + 0.30192500000000000001e0 * t23481 - 0.60384999999999999999e0 * t29088 + 0.181155e1 * t29094;
    let t29152 = t1248 * t11003 * t28369;
    (t29139, t29146, t29152)
}
