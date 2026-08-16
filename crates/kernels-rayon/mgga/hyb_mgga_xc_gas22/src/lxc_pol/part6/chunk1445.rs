//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1445/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1445(t1145: f64, t2876: f64, t4524: f64, t11474: f64, t11512: f64, t22640: f64, t22645: f64, t22653: f64, t22858: f64, t2868: f64, t2869: f64, t2889: f64, t31419: f64, t31436: f64, t3713: f64, t3717: f64, t3724: f64, t3753: f64, t4530: f64, t4540: f64, t9458: f64, t9538: f64, t9681: f64) -> f64 {
    let t31448 = t1145 * t4524 * t2876;
    let t31477 = -80.0_f64 / 3.0_f64 * t9538 * t11474 * t9681 + 1408.0_f64 / 243.0_f64 * t3753 * t31436 + 60.0_f64 * t22645 * t31448 + 180.0_f64 * t22653 * t1145 * t4530 * t2869 + 630.0_f64 * t22858 * t1145 * t4530 * t2876 - 6400.0_f64 / 27.0_f64 * t3724 * t31419 + 1600.0_f64 / 27.0_f64 * t11512 * t9458 - 6400.0_f64 / 81.0_f64 * t3713 * t31419 - 6400.0_f64 / 81.0_f64 * t3717 * t31419 + 630.0_f64 * t22858 * t1145 * t4524 * t2869 + 1512.0_f64 * t22640 * t31448 + 15.0_f64 * t2868 * t1145 * t4540 * t2889;
    t31477
}
