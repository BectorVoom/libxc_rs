//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1160/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1160<F: Float>(t35373: F, t792: F, t37327: F, t4176: F, t10615: F, t12428: F, t3275: F, t11483: F, t11523: F, t910: F, t983: F, t481: F) -> (F, F, F, F, F) {
    let t42868 = t35373 * t792;
    let t42871 = F::new(15.0) / F::new(8.0) * t37327 * t4176 * t42868;
    let t42874 = F::new(5.0) / F::new(16.0) * t3275 * t10615 * t12428;
    let t42876 = t11523 * t11483 / F::new(2.0);
    let t42877 = t910 * t983;
    let t42878 = t42877 * t481;
    (t42871, t42874, t42876, t42877, t42878)
}
