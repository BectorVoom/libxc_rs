//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 672/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk672<F: Float>(t19977: F, t35: F, t4474: F, t930: F, t374: F, t4449: F, t938: F, t1631: F, t4466: F, t929: F, t1594: F, t4467: F) -> (F, F, F, F, F, F, F, F) {
    let t19978 = t19977 * t35;
    let t19982 = t930 * t4474;
    let t19983 = t374 * t19982;
    let t19986 = t4449 * t938;
    let t19987 = t1631 * t19986;
    let t19993 = t4466 * t929;
    let t19994 = t19993 * t35;
    let t19995 = t1594 * t19994;
    let t19998 = t1594 * t19986;
    let t20004 = t1631 * t19994;
    let t20007 = t4467 * t938;
    (t19978, t19983, t19987, t19993, t19995, t19998, t20004, t20007)
}
