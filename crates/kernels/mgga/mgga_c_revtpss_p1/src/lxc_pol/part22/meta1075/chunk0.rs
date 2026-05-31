//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3855/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3855<F: Float>(t48287: F, t48290: F, t48292: F, t48294: F, t187: F, t73472: F, t48297: F, t48299: F, t48302: F, t48304: F, t48306: F, t47089: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t74114 = F::cast_from(48.0_f64) * t48287;
    let t74115 = F::cast_from(96.0_f64) * t48290;
    let t74116 = F::cast_from(160.0_f64) * t48292;
    let t74117 = F::cast_from(240.0_f64) * t48294;
    let t74119 = F::cast_from(0.19751673498613801407e-1_f64) * t73472 * t187;
    let t74120 = F::cast_from(0.20508037716432813315e4_f64) * t48297;
    let t74121 = F::cast_from(0.69263436422725855034e2_f64) * t48299;
    let t74122 = F::cast_from(0.43374325201206959368e-1_f64) * t48302;
    let t74123 = F::cast_from(0.32530743900905219526e-1_f64) * t48304;
    let t74124 = F::cast_from(0.96319466275353142155e0_f64) * t48306;
    let t74125 = F::cast_from(8.0_f64) * t47089;
    (t74114, t74115, t74116, t74117, t74119, t74120, t74121, t74122, t74123, t74124, t74125)
}
