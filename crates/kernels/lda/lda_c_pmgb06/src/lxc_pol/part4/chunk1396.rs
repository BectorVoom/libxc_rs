//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1396/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1396<F: Float>(t16100: F, t16101: F, t16102: F, t16103: F, t16105: F, t16107: F, t16109: F, t16112: F, t16114: F, t16117: F, t16121: F, t16122: F, t9424: F, t9426: F, t9429: F) -> F {
    let t18206 = 4e-21 * t9424 + F::new(16.0) / F::new(81.0) * t9426 + t9429 - t16100 + t16101 - t16102 - t16103 - t16105 - t16107 + t16109 + t16112 + t16114 + t16117 + t16121 + t16122;
    t18206
}
