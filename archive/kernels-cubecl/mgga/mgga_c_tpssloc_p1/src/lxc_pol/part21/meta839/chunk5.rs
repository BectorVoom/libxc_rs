//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3006/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3006<F: Float>(t10422: F, t18020: F, t3070: F, t10883: F, t13969: F, t17979: F, t17620: F, t2960: F, t10390: F, t17649: F, t17980: F, t17984: F, t3146: F, t42565: F, t43211: F, t43307: F, t43325: F, t43336: F, t43341: F, t50343: F, t50361: F, t50378: F, t50384: F, t55723: F, t973: F, t974: F) -> F {
    let t62811 = t3070 * t10422 * t18020;
    let t62816 = t10883 * t13969 * t17979;
    let t62827 = t2960 * t17620;
    let t62829 = F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t50343 - t50361 / F::cast_from(324.0_f64) - t43307 + t50378 / F::cast_from(1728.0_f64) - t10390 * t17649 / F::cast_from(1152.0_f64) - t50384 / F::cast_from(324.0_f64) + t62811 / F::cast_from(3456.0_f64) + t42565 * t17984 / F::cast_from(48.0_f64) + t62816 / F::cast_from(2304.0_f64) - t43211 * t17980 / F::cast_from(288.0_f64) + t973 * t974 * t3146 * t55723 / F::cast_from(108.0_f64) + t43325 / F::cast_from(243.0_f64) + t43336 / F::cast_from(10368.0_f64) - F::cast_from(5.0_f64) / F::cast_from(62208.0_f64) * t43341 - F::cast_from(2.0_f64) / F::cast_from(243.0_f64) * t62827;
    t62829
}
