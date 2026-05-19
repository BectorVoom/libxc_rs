//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 617/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk617<F: Float>(t7584: F, t7588: F, t7592: F, t7597: F, t7601: F, t7604: F, t7606: F, t7608: F, t7610: F, t7612: F, t7615: F, t7618: F, t7620: F, t7622: F, t8125: F, t8129: F) -> F {
    let t8141 = t8125 + F::cast_from(0.79656924630363488034e-2_f64) * t7584 + F::cast_from(0.1814407727691612783e-3_f64) * t7588 - F::cast_from(0.21168090156402149135e-3_f64) * t7592 + t8129 + F::cast_from(0.10643770401656718724e0_f64) * t7597 - F::cast_from(0.5454932330849068346e-1_f64) * t7601 - F::cast_from(0.25401708187682578962e-2_f64) * t7604 - F::cast_from(0.19957069503106347607e-1_f64) * t7606 + F::cast_from(0.2993560425465952141e-1_f64) * t7608 - F::cast_from(0.66380770525302906695e-3_f64) * t7610 + F::cast_from(0.79656924630363488034e-3_f64) * t7612 - F::cast_from(0.11974241701863808564e0_f64) * t7615 - F::cast_from(0.15965655602485078085e0_f64) * t7618 + F::cast_from(0.35403077613494883571e-2_f64) * t7620 - F::cast_from(0.55759847241254441624e-2_f64) * t7622;
    t8141
}
