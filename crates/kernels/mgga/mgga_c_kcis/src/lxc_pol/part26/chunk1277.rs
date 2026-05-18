//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1277/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1277<F: Float>(t12844: F, t27583: F, t29582: F, t18171: F, t29574: F, t27567: F, t101868: F, t101871: F, t101875: F, t101884: F, t27636: F, t28755: F, t28767: F, t28816: F, t28853: F, t6176: F, t77762: F, t7968: F, t7978: F, t99024: F, t99248: F, t99331: F) -> (F, F) {
    let t101892 = t27583 * t12844 * t29582;
    let t101894 = t18171 * t29574;
    let t101895 = t27567 * t101894;
    let t101898 = -F::new(0.82448622685185185186e-4) * t99248 * t28755 + F::new(0.8237654320987654321e-3) * t99331 * t28767 - F::new(0.23214722222222222221e-2) * t101868 + F::new(0.19345601851851851852e-2) * t101871 + F::new(0.208515625e-2) * t7978 * t101875 - F::new(0.69505208333333333334e-3) * t7978 * t6176 * t27636 * t77762 - F::new(0.34752604166666666667e-3) * t7978 * t101884 - F::new(0.46377350260416666667e-4) * t7968 * t101884 + F::new(0.24734586805555555555e-3) * t28853 * t28816 + F::new(0.7722800925925925926e-4) * t101892 + F::new(0.10306077835648148148e-4) * t101895 - F::new(0.30918233506944444445e-4) * t99024;
    (t101894, t101898)
}
