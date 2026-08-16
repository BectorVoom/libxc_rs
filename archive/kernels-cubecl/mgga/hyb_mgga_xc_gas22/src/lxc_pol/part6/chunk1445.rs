//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1445/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1445<F: Float>(t1145: F, t2876: F, t4524: F, t11474: F, t11512: F, t22640: F, t22645: F, t22653: F, t22858: F, t2868: F, t2869: F, t2889: F, t31419: F, t31436: F, t3713: F, t3717: F, t3724: F, t3753: F, t4530: F, t4540: F, t9458: F, t9538: F, t9681: F) -> F {
    let t31448 = t1145 * t4524 * t2876;
    let t31477 = -F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t9538 * t11474 * t9681 + F::cast_from(1408.0_f64) / F::cast_from(243.0_f64) * t3753 * t31436 + F::cast_from(60.0_f64) * t22645 * t31448 + F::cast_from(180.0_f64) * t22653 * t1145 * t4530 * t2869 + F::cast_from(630.0_f64) * t22858 * t1145 * t4530 * t2876 - F::cast_from(6400.0_f64) / F::cast_from(27.0_f64) * t3724 * t31419 + F::cast_from(1600.0_f64) / F::cast_from(27.0_f64) * t11512 * t9458 - F::cast_from(6400.0_f64) / F::cast_from(81.0_f64) * t3713 * t31419 - F::cast_from(6400.0_f64) / F::cast_from(81.0_f64) * t3717 * t31419 + F::cast_from(630.0_f64) * t22858 * t1145 * t4524 * t2869 + F::cast_from(1512.0_f64) * t22640 * t31448 + F::cast_from(15.0_f64) * t2868 * t1145 * t4540 * t2889;
    t31477
}
