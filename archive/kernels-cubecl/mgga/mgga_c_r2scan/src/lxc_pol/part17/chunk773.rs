//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 773/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk773<F: Float>(t1632: F, t2719: F, t551: F, t549: F, t2169: F, t2731: F, t2236: F, t2727: F, t2219: F, t2670: F, t2177: F, t2699: F) -> (F, F, F, F, F, F) {
    let t7390 = t1632 * t2719;
    let t7391 = t551 * t7390;
    let t7393 = F::cast_from(0.23115257973478049502e0_f64) * t549 * t7391;
    let t7395 = F::cast_from(0.69345773920434148506e0_f64) * t2169 * t2731;
    let t7397 = F::cast_from(0.23115257973478049502e0_f64) * t2236 * t2727;
    let t7399 = F::cast_from(0.69345773920434148506e0_f64) * t2670 * t2219;
    let t7401 = F::cast_from(0.25610080155860322884e0_f64) * t2177 * t2699;
    (t7390, t7393, t7395, t7397, t7399, t7401)
}
