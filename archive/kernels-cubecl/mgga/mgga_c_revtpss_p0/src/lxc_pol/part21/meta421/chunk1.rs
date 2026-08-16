//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1911/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1911<F: Float>(t2516: F, t5571: F, t5566: F, t72: F, t757: F, t1320: F, t5567: F, t5569: F, t9395: F, t9398: F, t1353: F, t1448: F) -> (F, F, F, F, F, F, F, F) {
    let t13611 = t5571 * t2516;
    let t13612 = F::cast_from(0.5848223622634646207e0_f64) * t13611;
    let t13613 = t5566 * t72;
    let t13615 = F::cast_from(0.36622894612013090108e-3_f64) * t13613 * t757;
    let t13620 = F::cast_from(8.0_f64) * t1320 * t5567;
    let t13621 = t1320 * t5569;
    let t13622 = F::cast_from(8.0_f64) * t13621;
    let t13623 = F::cast_from(4.0_f64) * t9395;
    let t13624 = F::cast_from(16.0_f64) * t9398;
    let t13625 = t1353 * t1448;
    (t13612, t13613, t13615, t13620, t13622, t13623, t13624, t13625)
}
