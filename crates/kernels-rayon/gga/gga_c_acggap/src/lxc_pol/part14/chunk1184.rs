//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1184/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1184(t1967: f64, t9549: f64, t2001: f64, t5975: f64, t31658: f64, t31660: f64, t31663: f64, t35916: f64, t35918: f64, t35920: f64, t35927: f64, t35931: f64, t35935: f64, t37786: f64, t40308: f64, t40310: f64, t40313: f64, t40316: f64, t40318: f64, t40322: f64) -> f64 {
    let t40324 = t1967 * t9549;
    let t40326 = t2001 * t5975;
    let t40328 = t35916 - t35918 + t35920 - 0.41930789719472202756e-2_f64 * t31658 + 0.47172138434406228102e-3_f64 * t31660 + t31663 + 0.85748036236139473944e-3_f64 * t40308 - 0.40015750243531754507e-2_f64 * t40310 + t37786 + t40313 / 24.0_f64 + t40316 / 24.0_f64 + t35927 + 0.17149607247227894789e-2_f64 * t40318 - 0.53592522647587171215e-3_f64 * t40322 + 0.64311027177104605458e-2_f64 * t40324 - t35931 - t35935 + 0.85748036236139473945e-2_f64 * t40326;
    t40328
}
