//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 744/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk744<F: Float>(t1390: F, t4057: F, t828: F, t1389: F, t1408: F, t2736: F, t1388: F, t1410: F, t3970: F, t3976: F, t3982: F, t3987: F, t3990: F, t3996: F, t4002: F, t4006: F, t4014: F, t4022: F) -> (F, F, F, F) {
    let t4059 = t1390 * t828 * t4057;
    let t4062 = t1408 * t1389;
    let t4064 = 0.25410001404642664112e-5 * t2736 * t4062;
    let t4065 = -0.85748036236139473944e-3 * t1410 * t3970 - t3976 - 0.10164000561857065645e-3 * t3982 + t3987 + 0.80031500487063509015e-2 * t3990 + 0.14291339372689912324e-4 * t3996 + 0.42874018118069736972e-3 * t4002 * t4006 + 0.42874018118069736972e-2 * t1410 * t4014 - 0.25410001404642664112e-4 * t4022 - 0.21437009059034868486e-3 * t1388 * t4059 - t4064;
    (t4059, t4062, t4064, t4065)
}
