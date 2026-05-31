//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1011/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1011<F: Float>(t1117: F, t1134: F, t1145: F, t1546: F, t2869: F, t2876: F, t2889: F, t2893: F, t2903: F, t3713: F, t3717: F, t3760: F, t3767: F, t3772: F, t3788: F, t518: F, t7721: F, t7739: F, t7769: F, t7780: F, t9419: F, t9436: F, t9441: F, t9444: F, t9449: F, t9453: F, t9458: F) -> F {
    let t9463 = -F::cast_from(4.0_f64) * t1117 * t9419 - F::cast_from(36.0_f64) * t1134 * t3760 * t2893 - F::cast_from(36.0_f64) * t1134 * t1546 * t2889 + F::cast_from(42.0_f64) * t518 * t3767 * t2893 - F::cast_from(8.0_f64) * t3788 * t3772 - F::cast_from(180.0_f64) * t2903 * t1546 * t2869 - F::cast_from(336.0_f64) * t518 * t9436 * t2876 + F::cast_from(12.0_f64) * t1117 * t9441 - F::cast_from(90.0_f64) * t7721 * t1145 * t9444 - F::cast_from(168.0_f64) * t7780 * t9449 + F::cast_from(6.0_f64) * t7739 * t9453 - F::cast_from(12.0_f64) * t7769 * t9449 + F::cast_from(800.0_f64) / F::cast_from(27.0_f64) * t3713 * t9458 + F::cast_from(800.0_f64) / F::cast_from(27.0_f64) * t3717 * t9458;
    t9463
}
