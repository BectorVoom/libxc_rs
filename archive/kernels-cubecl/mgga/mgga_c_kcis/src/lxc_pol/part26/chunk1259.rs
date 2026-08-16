//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1259/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1259<F: Float>(t7978: F, t99059: F, t18171: F, t27583: F, t28759: F, t18175: F, t28766: F, t11418: F, t1616: F, t27607: F, t28778: F, t54162: F, t8225: F) -> (F, F, F, F, F, F) {
    let t99100 = F::cast_from(0.46336805555555555556e-3_f64) * t7978 * t99059;
    let t99108 = F::cast_from(0.15445601851851851852e-3_f64) * t27583 * t18171 * t28759;
    let t99117 = F::cast_from(0.10297067901234567901e-3_f64) * t27583 * t18175 * t28766;
    let t99120 = t1616 * t11418;
    let t99129 = F::cast_from(0.23168402777777777778e-3_f64) * t27607 * t28778;
    let t99131 = t7978 * t54162 * t8225;
    (t99100, t99108, t99117, t99120, t99129, t99131)
}
