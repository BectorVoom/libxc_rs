//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1068/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1068<F: Float>(t1056: F, t13462: F, t13467: F, t345: F, t4910: F, t733: F, t4913: F, t2630: F, t4566: F) -> (F, F, F, F, F) {
    let t13485 = t1056 * t13462;
    let t13488 = t345 * t13467;
    let t13492 = F::cast_from(0.18736e-1_f64) * t733 * t4910;
    let t13493 = t733 * t4913;
    let t13495 = t4566 * t2630;
    (t13485, t13488, t13492, t13493, t13495)
}
