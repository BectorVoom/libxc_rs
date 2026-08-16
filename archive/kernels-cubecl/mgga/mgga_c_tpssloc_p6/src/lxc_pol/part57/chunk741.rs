//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 741/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk741<F: Float>(t112: F, t7758: F, t12461: F, t2094: F, t193: F, t200: F, t2056: F, t25049: F, t25277: F, t25077: F, t25080: F, t25140: F) -> (F, F, F, F, F, F, F, F) {
    let t26523 = t7758 * t112;
    let t26558 = t2094 * t12461;
    let t26563 = t193 * t200 * t2056;
    let t26591 = F::cast_from(0.38381794893125283518e-1_f64) * t25049;
    let t26613 = F::cast_from(0.38381794893125283518e-1_f64) * t25277;
    let t26619 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t25077;
    let t26621 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t25080;
    let t26644 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t25140;
    (t26523, t26558, t26563, t26591, t26613, t26619, t26621, t26644)
}
