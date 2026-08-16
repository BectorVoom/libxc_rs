//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1044/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1044(t6742: f64, t6751: f64, t1796: f64, t509: f64, t6636: f64, t6739: f64, t6642: f64, t1772: f64, t1998: f64, t6748: f64, t1994: f64, t6814: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22699 = t6742 * t6751;
    let t22700 = 0.1926377843805564792e1_f64 * t22699;
    let t22703 = 0.38024868119570572865e2_f64 * t1796 * t509 * t6636;
    let t22704 = t6742 * t6739;
    let t22705 = 0.65061485296689145286e-1_f64 * t22704;
    let t22708 = 0.21687161765563048428e-1_f64 * t1796 * t509 * t6642;
    let t22711 = 0.43374323531126096856e-1_f64 * t1796 * t1772 * t1998;
    let t22712 = t6742 * t6748;
    let t22713 = 0.86748647062252193714e-1_f64 * t22712;
    let t22716 = 0.1284251895870376528e1_f64 * t1796 * t1772 * t1994;
    let t22719 = 0.38527556876111295841e1_f64 * t1796 * t509 * t6814;
    (t22700, t22703, t22705, t22708, t22711, t22713, t22716, t22719)
}
