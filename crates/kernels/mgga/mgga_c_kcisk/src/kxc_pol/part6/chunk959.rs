//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 959/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk959<F: Float>(t24561: F, t2647: F, t1994: F, t23874: F, t23876: F, t23878: F, t23880: F, t23894: F, t28726: F, t28732: F, t28752: F, t28758: F, t28762: F, t28765: F) -> (F, F) {
    let t29981 = t24561 * t2647;
    let t29988 = -F::cast_from(0.92858888888888888888e-2_f64) * t28726 + F::new(0.10446625e-1) * t28732 + F::cast_from(0.23214722222222222222e-2_f64) * t23874 - F::cast_from(0.69644166666666666665e-2_f64) * t23876 - F::cast_from(0.77382407407407407405e-3_f64) * t23878 - F::cast_from(0.12381185185185185185e-1_f64) * t23880 - F::cast_from(0.34822083333333333333e-2_f64) * t23894 + F::new(0.579e0) * t1994 * t29981 + F::new(0.10446625e-1) * t28752 + F::cast_from(0.11607361111111111111e-2_f64) * t28758 + F::cast_from(0.51588271604938271605e-2_f64) * t28762 + F::cast_from(0.34822083333333333333e-2_f64) * t28765;
    (t29981, t29988)
}
