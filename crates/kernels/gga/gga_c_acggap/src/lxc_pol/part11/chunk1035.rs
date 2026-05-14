//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1035/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1035<F: Float>(t36081: F, t30090: F, t8897: F, t31362: F, t8903: F, t7839: F, t8908: F, t8912: F, t1165: F, t2068: F, t35102: F, t7351: F, t31697: F, t31702: F, t31704: F, t31721: F, t36063: F, t36066: F, t36068: F, t36070: F, t36072: F, t36075: F, t36077: F) -> (F,) {
    let t36082 = 0.62896184579208304136e-3 * t36081;
    let t36083 = t30090 * t8897;
    let t36085 = t31362 * t8903;
    let t36086 = 0.10718504529517434243e-2 * t36085;
    let t36087 = t7839 * t8908;
    let t36088 = 0.42874018118069736972e-3 * t36087;
    let t36089 = t7839 * t8912;
    let t36090 = 0.21437009059034868486e-3 * t36089;
    let t36093 = t2068 * t1165 * t7351 * t35102;
    let t36095 = t36063 / 48.0 - t36066 + t36068 / 64.0 + t36070 + 0.53592522647587171215e-3 * t31697 - t36072 + 0.31448092289604152068e-3 * t31702 + 0.41930789719472202756e-3 * t31704 + t36075 + 0.18868855373762491241e-2 * t36077 + t36082 - t31721 + 0.21437009059034868486e-3 * t36083 + t36086 + t36088 - t36090 - 0.47172138434406228102e-3 * t36093;
    (t36095,)
}
