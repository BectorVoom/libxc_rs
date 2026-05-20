//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2132;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2133;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta620<F: Float>(t1940: F, t2255: F, t7087: F, t27383: F, t61155: F, t27375: F, t92790: F, t14767: F, t27159: F, t4537: F, t605: F, t15071: F, t30: F, t25207: F, t61203: F, t4433: F, t892: F, t14749: F, t198: F, t7188: F, t11064: F, t7782: F, t1468: F, t2403: F, t25206: F, t25436: F, t25446: F, t25452: F, t27158: F, t27173: F, t27368: F, t27385: F, t51780: F, t7091: F, t7750: F) -> (F, F, F, F) {
        let (t98684, t98688, t98694, t98699, t98702, t98705) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2132::<F>(t1940, t2255, t7087, t27383, t61155, t27375, t92790, t14767, t27159, t4537, t605, t15071, t30);
        let (t98719, t98722, t98725) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2133::<F>(t25207, t61203, t4433, t605, t892, t14749, t27159, t198, t7188, t11064, t7782, t1468, t1940, t2403, t25206, t25436, t25446, t25452, t27158, t27173, t27368, t27385, t51780, t7087, t7091, t7750, t98684, t98688, t98694, t98699, t98702, t98705);
    (t98684, t98719, t98722, t98725)
}
