//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 766/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk766(t14185: f64, t17859: f64, t14189: f64, t14193: f64, t14199: f64, t3154: f64, t38472: f64, t1971: f64, t2367: f64, t495: f64, t515: f64, t7230: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t73873 = t17859 * t14185;
    let t73875 = t17859 * t14189;
    let t73877 = t17859 * t14193;
    let t73879 = t17859 * t14199;
    let t73881 = t38472 * t3154;
    let t73887 = 0.1064114997332445985e-4_f64 * t7230 * t1971 * t515 * t2367 * t495;
    (t73873, t73875, t73877, t73879, t73881, t73887)
}
