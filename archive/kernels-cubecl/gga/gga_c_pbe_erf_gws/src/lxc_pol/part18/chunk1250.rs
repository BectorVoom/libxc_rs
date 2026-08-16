//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1250/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1250<F: Float>(t14784: F, t50994: F, t20091: F, t4157: F, t3202: F, t3955: F, t14113: F, t14614: F, t2242: F, t4161: F, t14742: F, t840: F) -> (F, F, F, F, F, F) {
    let t53952 = t50994 * t14784;
    let t53953 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53952;
    let t53959 = t20091 * t4157;
    let t53970 = t3955 * t3202;
    let t53971 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t53970;
    let t53975 = t14113 * t14614;
    let t53976 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53975;
    let t53977 = t2242 * t4161;
    let t53980 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t840 * t14742;
    (t53953, t53959, t53971, t53976, t53977, t53980)
}
