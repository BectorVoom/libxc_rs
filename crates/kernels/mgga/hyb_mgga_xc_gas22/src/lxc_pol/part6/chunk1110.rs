//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1110/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1110<F: Float>(t10898: F, t10913: F, t6969: F, t7016: F, t9008: F, t9134: F, t950: F, t4247: F, t7025: F, t952: F, t3490: F, t3496: F) -> (F, F, F, F, F) {
    let t10914 = -t7016 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t6969 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9008 - t9134 - t10898 / F::cast_from(3.0_f64) + t10913;
    let t10915 = t950 * t10914;
    let t10921 = t7025 * t4247;
    let t10922 = t10921 * t952;
    let t10924 = t3496 * t3490;
    (t10914, t10915, t10921, t10922, t10924)
}
