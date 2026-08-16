//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 959/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk959(t24561: f64, t2647: f64, t1994: f64, t23874: f64, t23876: f64, t23878: f64, t23880: f64, t23894: f64, t28726: f64, t28732: f64, t28752: f64, t28758: f64, t28762: f64, t28765: f64) -> (f64, f64) {
    let t29981 = t24561 * t2647;
    let t29988 = -0.92858888888888888888e-2_f64 * t28726 + 0.10446625e-1_f64 * t28732 + 0.23214722222222222222e-2_f64 * t23874 - 0.69644166666666666665e-2_f64 * t23876 - 0.77382407407407407405e-3_f64 * t23878 - 0.12381185185185185185e-1_f64 * t23880 - 0.34822083333333333333e-2_f64 * t23894 + 0.579e0_f64 * t1994 * t29981 + 0.10446625e-1_f64 * t28752 + 0.11607361111111111111e-2_f64 * t28758 + 0.51588271604938271605e-2_f64 * t28762 + 0.34822083333333333333e-2_f64 * t28765;
    (t29981, t29988)
}
