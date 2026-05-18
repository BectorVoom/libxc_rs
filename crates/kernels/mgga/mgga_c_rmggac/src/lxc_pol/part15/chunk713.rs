//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 713/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk713<F: Float>(t10002: F, t10120: F, t10124: F, t10130: F, t10135: F, t10137: F, t10141: F, t10143: F, t10148: F, t10151: F, t118: F, t305: F, t326: F, t4669: F, t5148: F, t7819: F, t7821: F, t793: F, t8911: F, t8913: F, t8917: F, t9840: F, t9858: F, t9867: F, t9944: F) -> F {
    let t10153 = F::new(0.54549323308490683457e-1) * t8911 - F::new(0.72732431077987577943e-1) * t8913 - F::new(0.18183107769496894486e-1) * t8917 + F::new(0.13637330827122670864e-1) * t10120 + F::new(0.34093327067806677161e-2) * t10124 + F::new(0.11974241701863808564e0) * t305 * t9858 + F::new(0.11974241701863808564e0) * t793 * t9840 - F::new(0.23948483403727617128e0) * t5148 * t10130 + F::new(0.59871208509319042821e-1) * t305 * t10002 - F::new(0.17961362552795712846e0) * t10135 - F::new(0.5987120850931904282e-1) * t10137 - F::new(0.79828278012425390428e-1) * t118 * t9867 + F::new(0.17961362552795712846e0) * t10141 - F::new(0.35922725105591425692e0) * t4669 * t10143 + t7819 - t7821 - F::new(0.11974241701863808564e0) * t326 * t9944 - F::new(0.59871208509319042821e-1) * t326 * t10148 + F::new(0.2993560425465952141e-1) * t10151;
    t10153
}
