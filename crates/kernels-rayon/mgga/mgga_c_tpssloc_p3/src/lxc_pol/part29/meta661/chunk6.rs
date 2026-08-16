//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2201/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2201(t90701: f64, t1985: f64, t22662: f64, t26193: f64, t81284: f64, t26203: f64, t6883: f64, t6897: f64, t7700: f64, t80645: f64, t12030: f64, t1375: f64, t1385: f64, t16022: f64, t16436: f64, t16474: f64, t16475: f64, t1843: f64, t2015: f64, t2016: f64, t26224: f64, t26348: f64, t26371: f64, t26471: f64, t26477: f64, t3758: f64, t3887: f64, t3912: f64, t55069: f64, t55134: f64, t6958: f64, t6993: f64, t7750: f64, t81282: f64, t81319: f64, t90687: f64, t90690: f64, t90696: f64) -> f64 {
    let t90702 = 0.82246703342411321824e-2_f64 * t90701;
    let t90704 = t1985 * t26193 * t22662;
    let t90706 = 0.3289868133696452873e-1_f64 * t81284;
    let t90707 = t6883 * t26203;
    let t90708 = 0.38381794893125283518e-1_f64 * t90707;
    let t90723 = t6897 * t80645 * t7700;
    let t90724 = 0.82246703342411321824e-2_f64 * t90723;
    let t90725 = 2.0_f64 * t1375 * t3887 * t2015 * t16436 - t90687 - 0.82246703342411321825e-2_f64 * t90690 + 4.0_f64 * t1375 * t3887 * t26471 * t1385 + t81282 + 24.0_f64 * t26224 * t90696 * t16474 + t90702 - 0.82246703342411321825e-2_f64 * t90704 + t90706 + t90708 - t26477 * t3912 - t81319 * t1843 + 4.0_f64 * t3758 * t26348 - t55069 * t2016 - t55134 * t2016 - 2.0_f64 * t16022 * t6993 - t12030 * t7750 + 4.0_f64 * t3758 * t26371 - 6.0_f64 * t6958 * t16475 + t90724;
    t90725
}
