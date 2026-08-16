//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1247/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1247<F: Float>(t11047: F, t2197: F, t2028: F, t3038: F, t7275: F, t787: F, t10012: F, t10627: F, t15482: F, t22633: F, t11053: F, t7419: F, t9805: F) -> (F, F, F, F) {
    let t33136 = F::cast_from(0.23005755572352449806e2_f64) * t2197 * t11047;
    let t33145 = F::cast_from(0.79445533226334281486e-1_f64) * t787 * t7275 * t3038 * t2028;
    let t33148 = t10012 * t10627;
    let t33151 = F::cast_from(0.5680433474654925878e0_f64) * t22633 * t15482 * t33148;
    let t33153 = t9805 * t11053 * t7419;
    (t33136, t33145, t33151, t33153)
}
