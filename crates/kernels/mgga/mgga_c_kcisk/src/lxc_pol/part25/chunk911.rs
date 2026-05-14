//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 911/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk911<F: Float>(t16099: F, t4658: F, t11460: F, t11461: F, t11463: F, t11465: F, t11467: F, t11488: F, t11491: F, t11493: F, t11495: F, t16077: F, t16081: F, t16084: F, t16085: F, t16088: F, t16090: F, t16092: F, t16096: F, t1809: F, t1860: F, t5089: F, t5134: F, t674: F, t682: F, t6884: F) -> (F,) {
    let t16100 = t16099 * t4658;
    let t16103 = 0.46853067927761790996e-2 * t5134 * t682 - 2.0 * t6884 * t1860 - 0.93706135855523581992e-2 * t11461 - 0.18741227171104716398e-1 * t11463 + 0.23426533963880895498e-2 * t11465 + 0.46853067927761790996e-2 * t11467 - 0.46853067927761790996e-2 * t11488 - 0.14055920378328537299e-1 * t11493 - 0.46853067927761790996e-2 * t674 * t16077 - 0.93706135855523581992e-2 * t11491 - 0.23426533963880895498e-1 * t16081 + t16084 - 0.14055920378328537299e-1 * t11495 * t16085 - 0.46853067927761790996e-2 * t16088 - 0.93706135855523581992e-2 * t16090 - t11460 - 0.93706135855523581992e-2 * t5089 * t16092 - 0.56223681513314149196e-1 * t674 * t16096 - 0.14055920378328537299e-1 * t1809 * t16100;
    (t16103,)
}
