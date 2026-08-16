//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2088/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2088<F: Float>(t91394: F, t91398: F, t91078: F, t91081: F, t91531: F, t91548: F, t1751: F, t7319: F, t1240: F, t5088: F, t11153: F, t497: F) -> (F, F, F, F, F, F, F, F, F) {
    let t93757 = F::cast_from(119.0_f64) / F::cast_from(3456.0_f64) * t91394;
    let t93760 = F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t91398;
    let t93795 = F::cast_from(0.52089578783527170489e-1_f64) * t91078;
    let t93796 = F::cast_from(0.3289868133696452873e-1_f64) * t91081;
    let t93899 = F::cast_from(0.52089578783527170489e-1_f64) * t91531;
    let t93906 = F::cast_from(0.3289868133696452873e-1_f64) * t91548;
    let t94297 = t7319 * t1751;
    let t94319 = t1240 * t5088;
    let t94349 = t497 * t11153;
    (t93757, t93760, t93795, t93796, t93899, t93906, t94297, t94319, t94349)
}
