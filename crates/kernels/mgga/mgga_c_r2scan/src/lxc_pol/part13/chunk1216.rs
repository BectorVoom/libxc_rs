//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1216/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1216<F: Float>(t40594: F, t40595: F, t4176: F, t10935: F, t2813: F, t3446: F, t11004: F, t11523: F, t6897: F, t983: F, t2330: F, t3275: F, t3276: F) -> (F, F, F, F) {
    let t40598 = F::new(45.0) / F::new(32.0) * t40594 * t4176 * t40595;
    let t40603 = t3446 * t10935 * t2813;
    let t40604 = F::cast_from(0.19211284388664477842e-2_f64) * t40603;
    let t40606 = F::new(5.0) / F::new(8.0) * t11523 * t11004;
    let t40608 = t6897 * t983;
    let t40609 = t40608 * t2330;
    let t40612 = F::new(5.0) / F::new(8.0) * t3275 * t3276 * t40609;
    (t40598, t40604, t40606, t40612)
}
