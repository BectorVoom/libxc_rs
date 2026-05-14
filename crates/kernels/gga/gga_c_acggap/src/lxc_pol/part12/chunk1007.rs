//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1007/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1007<F: Float>(t33874: F, t33886: F, t33894: F, t33903: F, t30037: F, t30061: F, t30073: F, t32339: F, t32340: F, t32341: F, t32342: F, t32348: F, t32349: F, t32350: F, t33876: F, t33881: F, t33890: F, t33898: F) -> (F,) {
    let t36833 = 0.10718504529517434243e-2 * t33874;
    let t36836 = 0.57165357490759649296e-3 * t33886;
    let t36838 = 0.28582678745379824648e-3 * t33894;
    let t36841 = 0.57165357490759649296e-3 * t33903;
    let t36842 = -0.13719685797782315831e-1 * t30037 + t32339 + t32340 + t32341 + t32342 + 0.42874018118069736972e-2 * t30061 - t36833 - 0.18007087609589289528e-1 * t33876 - 0.42874018118069736972e-3 * t33881 - t36836 - 0.85748036236139473944e-3 * t33890 - t36838 - 0.85748036236139473944e-3 * t33898 - 0.17149607247227894789e-2 * t30073 - t36841 - t32348 - t32349 + t32350;
    (t36842,)
}
