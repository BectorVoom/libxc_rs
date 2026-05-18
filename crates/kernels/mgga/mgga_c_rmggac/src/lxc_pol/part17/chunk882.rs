//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 882/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk882<F: Float>(t1502: F, t16503: F, t3369: F, t559: F, t34975: F, t34976: F, t571: F, t9145: F, t8537: F, t8659: F, t1665: F, t2010: F, t8342: F) -> (F, F, F, F, F) {
    let t44808 = t16503 * t3369 * t559 * t1502;
    let t44812 = t34975 * t34976 * t571 * t9145;
    let t44816 = t34975 * t3369 * t559 * t9145;
    let t44818 = t8659 * t8537;
    let t44821 = t2010 * t8342 * t1665;
    (t44808, t44812, t44816, t44818, t44821)
}
