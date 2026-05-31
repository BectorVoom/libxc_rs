//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2472/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2472<F: Float>(t48089: F, t221: F, t9817: F, t1320: F, t13632: F, t1317: F, t13680: F, t3860: F, t5567: F, t46971: F, t3857: F, t5569: F) -> (F, F, F, F, F, F, F, F) {
    let t48090 = F::cast_from(0.34697458558045176417e-2_f64) * t48089;
    let t48100 = t9817 * t221;
    let t48152 = t1320 * t13632;
    let t48157 = F::cast_from(24.0_f64) * t1317 * t13680;
    let t48158 = t3860 * t5567;
    let t48159 = F::cast_from(36.0_f64) * t48158;
    let t48224 = F::cast_from(480.0_f64) * t46971;
    let t48225 = t1317 * t13632;
    let t48226 = F::cast_from(12.0_f64) * t48225;
    let t48227 = t3857 * t5569;
    (t48090, t48100, t48152, t48157, t48159, t48224, t48226, t48227)
}
