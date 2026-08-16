//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 1043/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk1043(t12845: f64, t1421: f64, t26577: f64, t26579: f64, t26600: f64, t26602: f64, t31009: f64, t31013: f64, t31017: f64, t31021: f64, t31025: f64, t31060: f64, t31097: f64, t31131: f64, t456: f64) -> f64 {
    let t31133 = -0.26281718666666666667e-2_f64 * t26577 + 0.21901432222222222222e-2_f64 * t26579 - 0.59133867e-2_f64 * t26600 + 0.13140859333333333334e-2_f64 * t26602 + t12845 + 0.1478346675e-2_f64 * t1421 * t31009 - 0.59133867e-2_f64 * t1421 * t31013 + 0.39422577999999999999e-2_f64 * t1421 * t31017 + 0.39422577999999999999e-2_f64 * t1421 * t31021 - 0.36958666875e-3_f64 * t456 * t31025 + t31060 + t31097 + t31131;
    t31133
}
