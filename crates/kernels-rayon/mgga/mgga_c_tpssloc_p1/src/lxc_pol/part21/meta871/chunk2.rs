//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3202/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3202(t1244: f64, t3068: f64, t478: f64, t6163: f64, t11734: f64, t1227: f64, t15498: f64, t15525: f64, t15541: f64, t19072: f64, t3515: f64, t3580: f64, t4582: f64, t4977: f64, t4989: f64, t5024: f64, t52919: f64, t53456: f64, t53468: f64, t53470: f64, t53476: f64, t53481: f64, t53490: f64, t53494: f64, t53496: f64, t53498: f64, t61855: f64) -> f64 {
    let t66622 = t1244 * t478 * t6163 * t3068;
    let t66631 = -2.0_f64 / 243.0_f64 * t53456 - t53468 / 3456.0_f64 - t53470 / 1728.0_f64 - t53476 / 864.0_f64 - t53481 / 864.0_f64 - t11734 * t19072 / 768.0_f64 - t3515 * t4582 * t4977 * t15525 / 1536.0_f64 - 5.0_f64 / 648.0_f64 * t15498 * t4989 - 5.0_f64 / 648.0_f64 * t5024 * t15541 - 5.0_f64 / 243.0_f64 * t53490 - t53494 / 1728.0_f64 - 19.0_f64 / 1296.0_f64 * t66622 * t3580 + t53496 / 162.0_f64 + t53498 / 81.0_f64 + 55.0_f64 / 15552.0_f64 * t1227 * t4582 * t52919 * t61855;
    t66631
}
