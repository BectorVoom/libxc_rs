//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2080/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2080<F: Float>(t90645: F, t22716: F, t7701: F, t1834: F, t212: F, t22642: F, t6890: F, t26215: F, t81228: F, t81326: F, t2015: F, t40590: F) -> (F, F, F, F, F) {
    let t90646 = F::cast_from(0.82246703342411321824e-2_f64) * t90645;
    let t90659 = t22716 * t7701;
    let t90663 = t22642 * t212 * t1834 * t6890;
    let t90686 = t81228 * t81326 * t26215;
    let t90687 = F::cast_from(0.16449340668482264365e-1_f64) * t90686;
    let t90696 = t40590 * t2015;
    (t90646, t90659, t90663, t90687, t90696)
}
