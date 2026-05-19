//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1324/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1324<F: Float>(t102674: F, t102678: F, t102681: F, t102684: F, t102687: F, t102694: F, t27607: F, t28727: F, t28811: F, t29533: F, t95168: F, t99630: F, t99639: F, t99644: F) -> F {
    let t102696 = -t95168 - t99630 + F::cast_from(0.11607361111111111111e-2_f64) * t102674 + F::cast_from(0.15445601851851851852e-3_f64) * t99639 + F::cast_from(0.92858888888888888886e-2_f64) * t102678 + t99644 - F::cast_from(0.38691203703703703703e-3_f64) * t102681 + F::cast_from(0.77382407407407407407e-3_f64) * t102684 - F::cast_from(0.23214722222222222222e-2_f64) * t102687 + F::cast_from(0.23168402777777777778e-3_f64) * t27607 * t29533 + F::cast_from(0.37069444444444444444e-2_f64) * t28727 * t28811 + F::cast_from(0.77382407407407407407e-3_f64) * t102694;
    t102696
}
