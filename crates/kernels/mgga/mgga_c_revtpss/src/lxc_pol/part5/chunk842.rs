//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 842/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk842<F: Float>(t1041: F, t1063: F, t1671: F, t1675: F, t3150: F, t3161: F, t3203: F, t3205: F, t375: F, t4834: F, t4846: F, t4879: F, t4925: F, t6302: F, t6308: F, t6312: F, t6318: F, t6323: F, t6327: F, t6331: F, t6339: F) -> (F,) {
    let t6342 = 0.21437009059034868486e-3 * t1041 * t6302 + 0.42874018118069736972e-3 * t3150 * t6308 - 0.21437009059034868486e-3 * t3161 * t6312 + 0.42874018118069736972e-3 * t4879 * t1671 + 0.21437009059034868486e-3 * t6318 * t375 - 0.28582678745379824648e-3 * t4846 + 0.14291339372689912324e-3 * t1063 * t6323 + 0.23818898954483187207e-3 * t1063 * t6327 - 0.28582678745379824648e-3 * t1063 * t6331 - t3203 + t4925 / 432.0 + 0.28582678745379824648e-3 * t4834 * t1675 + 0.42874018118069736972e-3 * t3205 * t6339;
    (t6342,)
}
