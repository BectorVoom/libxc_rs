//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1147/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1147<F: Float>(t7630: F, t9268: F, t2153: F, t35635: F, t9276: F, t2539: F, t9275: F, t2770: F, t7655: F, t2161: F, t9016: F, t26439: F, t710: F, t86: F) -> (F, F, F, F, F, F) {
    let t91885 = F::cast_from(3.0_f64) * t9268 * t7630;
    let t91895 = F::cast_from(24.0_f64) * t35635 * t2153 * t9276;
    let t91901 = F::cast_from(18.0_f64) * t9275 * t7630 * t2539;
    let t91902 = t7655 * t2770;
    let t91905 = t2161 * t9016;
    let t91909 = t86 * t710 * t26439;
    (t91885, t91895, t91901, t91902, t91905, t91909)
}
