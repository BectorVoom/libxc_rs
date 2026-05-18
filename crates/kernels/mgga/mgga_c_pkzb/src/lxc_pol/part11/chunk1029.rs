//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1029/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1029<F: Float>(t11351: F, t11368: F, t405: F, t921: F, t758: F, t10059: F, t10061: F, t10081: F, t10132: F, t10136: F, t10197: F, t11338: F, t11342: F, t11348: F, t1230: F, t3174: F, t3877: F, t407: F, t6379: F, t6459: F, t8360: F, t918: F) -> (F, F, F) {
    let t11369 = t11351 + t11368;
    let t11371 = t405 * t11369 * t921;
    let t11372 = t758 * t11371;
    let t11381 = -F::new(0.45732285992607719436e-2) * t10059 + F::new(0.14481890564325777821e-1) * t10061 - F::new(0.17149607247227894789e-2) * t10081 + t6379 - F::new(0.53100265402527852012e-1) * t11338 * t407 + t3174 * t11342 / F::new(16.0) + F::new(0.21437009059034868486e-3) * t6459 * t11348 + F::new(0.21437009059034868486e-3) * t918 * t11372 + F::new(0.21722835846488666732e-1) * t10197 * t1230 + F::new(0.34299214494455789577e-2) * t8360 * t3877 + t10132 / F::new(48.0) - t10136 / F::new(96.0);
    (t11369, t11371, t11381)
}
