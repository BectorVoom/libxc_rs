//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 600/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk600<F: Float>(t1322: F, t4607: F, t2704: F, t2718: F, t4518: F, t4521: F, t4524: F, t4529: F, t4531: F, t4533: F, t456: F, t1314: F) -> (F, F, F, F) {
    let t4608 = t4607 * t1322;
    let t4619 = -F::cast_from(0.34523333333333333333e1_f64) * t4518 + F::cast_from(0.23015555555555555556e1_f64) * t4521 - F::cast_from(0.26851481481481481482e1_f64) * t4524 - F::cast_from(0.93932222222222222223e0_f64) * t2704 + F::cast_from(0.73355e-1_f64) * t4529 - F::cast_from(0.14671e0_f64) * t4531 - F::cast_from(0.17116166666666666667e0_f64) * t4533 - F::cast_from(0.36793333333333333333e0_f64) * t2718;
    let t4620 = t4619 * t456;
    let t4623 = t1314 * t1322;
    (t4608, t4619, t4620, t4623)
}
