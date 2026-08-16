//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 991/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk991(t10847: f64, t10851: f64, t10857: f64, t11428: f64, t11432: f64, t11433: f64, t11817: f64, t11819: f64, t11822: f64, t11826: f64, t11831: f64, t11843: f64) -> (f64, f64) {
    let t12188 = 0.47609969197673950973e-2_f64 * t11817 + 0.10975748638225852664e0_f64 * t11819 + 0.13099107994629972538e-1_f64 * t11822 + 0.13099107994629972538e-1_f64 * t11826 - t10847 - t10851 - t11428 - 0.97574405393827830187e-2_f64 * t10857 + 0.43663693315433241794e-2_f64 * t11831 + t11432 + t11433;
    let t12192 = 0.23115257973478049502e0_f64 * t11843;
    (t12188, t12192)
}
