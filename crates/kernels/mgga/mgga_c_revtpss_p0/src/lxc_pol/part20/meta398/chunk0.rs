//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1473/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1473<F: Float>(t11299: F, t2918: F, t2927: F, t11380: F, t2874: F, t934: F, t11379: F, t2924: F, t2926: F, t11294: F, t11531: F, t41500: F, t935: F) -> (F, F, F, F, F) {
    let t41864 = F::cast_from(0.57895126195293126241e3_f64) * t11299 * t2927 * t2918;
    let t41867 = F::cast_from(8.0_f64) * t2874 * t11380 * t934;
    let t41871 = F::cast_from(0.64327917994770140268e2_f64) * t2924 * t11379 * t2926 * t934;
    let t41873 = F::cast_from(24.0_f64) * t11294 * t11531;
    let t41876 = F::cast_from(24.0_f64) * t11299 * t41500 * t935;
    (t41864, t41867, t41871, t41873, t41876)
}
