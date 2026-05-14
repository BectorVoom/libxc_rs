//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1046/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1046<F: Float>(t2001: F, t5975: F, t31658: F, t31660: F, t31663: F, t35916: F, t35918: F, t35920: F, t35927: F, t35931: F, t35935: F, t37786: F, t40308: F, t40310: F, t40313: F, t40316: F, t40318: F, t40322: F, t40324: F) -> (F,) {
    let t40326 = t2001 * t5975;
    let t40328 = t35916 - t35918 + t35920 - 0.41930789719472202756e-2 * t31658 + 0.47172138434406228102e-3 * t31660 + t31663 + 0.85748036236139473944e-3 * t40308 - 0.40015750243531754507e-2 * t40310 + t37786 + t40313 / 24.0 + t40316 / 24.0 + t35927 + 0.17149607247227894789e-2 * t40318 - 0.53592522647587171215e-3 * t40322 + 0.64311027177104605458e-2 * t40324 - t35931 - t35935 + 0.85748036236139473945e-2 * t40326;
    (t40328,)
}
