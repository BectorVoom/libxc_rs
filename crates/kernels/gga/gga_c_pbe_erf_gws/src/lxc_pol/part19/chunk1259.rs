//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1259/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1259<F: Float>(t14959: F, t4414: F, t53545: F, t20091: F, t4209: F, t53577: F, t53583: F, t53597: F, t14911: F, t2367: F, t353: F, t4228: F, t4386: F, t810: F) -> (F, F, F, F, F, F, F, F) {
    let t55228 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t4414 * t14959;
    let t55238 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53545;
    let t55243 = t20091 * t4209;
    let t55248 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53577;
    let t55251 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53583;
    let t55258 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t53597;
    let t55279 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t2367 * t14911;
    let t55284 = t4386 * t353 * t4228 * t810;
    (t55228, t55238, t55243, t55248, t55251, t55258, t55279, t55284)
}
