//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 504/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk504<F: Float>(t234: F, t64: F, t2004: F, t2010: F, t2013: F, t2017: F, t2020: F, t44: F, t49: F, t56: F, t589: F, t592: F) -> (F, F, F) {
    let t2023 = t64 * t234;
    let t2024 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2023;
    let t2025 = F::cast_from(88.0_f64) / F::cast_from(9.0_f64) * t2004 * t49 - F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t589 * t592 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t44 * t2010 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t44 * t2013 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t56 * t2017 - F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t56 * t2020 - t2024;
    (t2023, t2024, t2025)
}
