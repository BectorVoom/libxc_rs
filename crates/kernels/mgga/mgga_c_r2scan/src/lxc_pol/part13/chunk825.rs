//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 825/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk825<F: Float>(t2236: F, t2727: F, t2219: F, t2670: F, t2177: F, t2699: F, t2526: F, t788: F, t2207: F, t785: F, t2841: F, t481: F) -> (F, F, F, F, F) {
    let t7397 = F::cast_from(0.23115257973478049502e0_f64) * t2236 * t2727;
    let t7399 = F::cast_from(0.69345773920434148506e0_f64) * t2670 * t2219;
    let t7401 = F::cast_from(0.25610080155860322884e0_f64) * t2177 * t2699;
    let t7402 = t788 * t2526;
    let t7405 = F::cast_from(0.34930954652346593434e-1_f64) * t2207 * t785 * t7402;
    let t7406 = t2841 * t481;
    (t7397, t7399, t7401, t7405, t7406)
}
