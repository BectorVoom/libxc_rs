//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 614/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk614<F: Float>(t300: F, t5797: F, t5770: F, t1589: F, t4483: F, t2904: F, t5774: F, t951: F, t959: F, t5790: F, t942: F, t2929: F) -> (F, F, F, F, F, F, F, F) {
    let t5798 = t300 * t5797;
    let t5800 = F::cast_from(0.19751673498613801407e-1_f64) * t300 * t5770;
    let t5802 = F::cast_from(0.11696447245269292414e1_f64) * t4483 * t1589;
    let t5804 = t2904 * t5774 * t951;
    let t5806 = F::cast_from(0.11696447245269292414e1_f64) * t959 * t5804;
    let t5808 = t942 * t5790 * t951;
    let t5810 = F::cast_from(0.5848223622634646207e0_f64) * t959 * t5808;
    let t5811 = t2929 * t5774;
    (t5798, t5800, t5802, t5804, t5806, t5808, t5810, t5811)
}
