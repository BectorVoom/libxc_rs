//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2330/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2330<F: Float>(t24987: F, t7754: F, t1983: F, t2019: F, t57806: F, t25971: F, t91655: F, t26161: F, t26162: F, t75210: F, t25994: F, t7458: F) -> (F, F, F, F, F) {
    let t100828 = F::cast_from(2.0_f64) * t24987 * t7754;
    let t100833 = t1983 * t2019 * t57806;
    let t100835 = F::cast_from(6.0_f64) * t91655 * t25971;
    let t100838 = F::cast_from(2.0_f64) * t26161 * t26162 * t75210;
    let t100840 = F::cast_from(4.0_f64) * t7458 * t25994;
    (t100828, t100833, t100835, t100838, t100840)
}
