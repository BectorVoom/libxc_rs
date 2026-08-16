//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 837/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk837(t1844: f64, t599: f64, t1181: f64, t2068: f64, t336: f64, t5630: f64, t570: f64, t7806: f64, t7850: f64, t7854: f64, t7863: f64, t8953: f64, t8975: f64, t8983: f64, t9348: f64, t9356: f64, t9359: f64, t9739: f64, t9741: f64, t9743: f64, t9747: f64, t9749: f64, t9751: f64, t9753: f64, t9755: f64) -> (f64, f64, f64, f64) {
    let t9757 = t599 * t1844;
    let t9758 = t1181 * t9757;
    let t9759 = t2068 * t9758;
    let t9761 = t336 * t5630;
    let t9762 = t570 * t9761;
    let t9764 = -t7806 - t9348 - 0.31448092289604152068e-3_f64 * t8953 - t9739 / 24.0_f64 - t9741 / 48.0_f64 + t9743 / 16.0_f64 + t9356 - 0.56606566121287473722e-2_f64 * t8975 - t9359 + 0.25724410870841842184e-2_f64 * t8983 - t9747 / 48.0_f64 - t9749 / 96.0_f64 + t9751 / 48.0_f64 + 0.85748036236139473945e-2_f64 * t9753 + 0.25724410870841842183e-2_f64 * t9755 + t7850 + t7854 - 0.10718504529517434243e-3_f64 * t9759 + t7863 + t9762 / 96.0_f64;
    (t9757, t9758, t9761, t9764)
}
