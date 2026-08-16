//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2984/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2984<F: Float>(t1036: F, t17878: F, t13969: F, t17631: F, t3039: F, t3082: F, t5905: F, t10937: F, t10952: F, t17632: F, t17677: F, t17960: F, t2986: F, t3070: F, t3071: F, t43110: F, t48585: F, t49889: F, t49892: F, t49894: F, t49897: F, t49906: F, t49922: F, t50370: F, t55716: F, t884: F) -> F {
    let t62343 = t17878 * t1036;
    let t62349 = t3039 * t13969 * t17631;
    let t62360 = t5905 * t3082;
    let t62362 = t3070 * t3071 * t17960 * t884 / F::cast_from(2304.0_f64) + t49889 / F::cast_from(162.0_f64) - F::cast_from(5.0_f64) / F::cast_from(972.0_f64) * t49892 - t49894 / F::cast_from(1152.0_f64) - t49897 / F::cast_from(1152.0_f64) + t49906 / F::cast_from(162.0_f64) + t62343 / F::cast_from(2304.0_f64) + t43110 / F::cast_from(648.0_f64) - t10937 * t17677 / F::cast_from(216.0_f64) - t62349 / F::cast_from(1152.0_f64) - t10952 * t17632 / F::cast_from(768.0_f64) - t49922 / F::cast_from(1728.0_f64) - t2986 * t50370 * t55716 / F::cast_from(9.0_f64) + F::cast_from(7.0_f64) / F::cast_from(162.0_f64) * t2986 * t48585 * t55716 - t62360 / F::cast_from(13824.0_f64);
    t62362
}
