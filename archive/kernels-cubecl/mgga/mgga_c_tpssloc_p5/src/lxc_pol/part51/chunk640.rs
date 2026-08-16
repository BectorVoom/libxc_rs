//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 640/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk640<F: Float>(t28: F, t1081: F, t5142: F, t5145: F, t584: F, t157: F, t5141: F, t182: F, t172: F, t1787: F, t763: F, t67: F, t758: F, zeta_threshold: F) -> (F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t5149 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t5142 * t1081 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t5145 * t584);
    let t5151 = (t5141 + t5149) * t157;
    let t5153 = F::cast_from(0.19751673498613801407e-1_f64) * t5151 * t182;
    let t5154 = t1787 * t172;
    let t5155 = t5154 * t763;
    let t5156 = F::cast_from(0.5848223622634646207e0_f64) * t5155;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    (t5151, t5153, t5156, t5158)
}
