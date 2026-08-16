//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1064/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1064<F: Float>(t13462: F, t945: F, t1646: F, t9372: F, t2630: F) -> (F, F) {
    let t13463 = t945 * t13462;
    let t13466 = t9372 * t1646;
    let t13467 = t13466 * t2630;
    (t13463, t13467)
}
