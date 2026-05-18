//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1004/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1004<F: Float>(t321: F, t35918: F, t35922: F, t35926: F, t35937: F, t41101: F, t41106: F, t41108: F, t41115: F, t41116: F, t41120: F, t41122: F, t4669: F, t5259: F, t833: F, t848: F, t876: F, t8936: F, t8975: F) -> F {
    let t41126 = -F::new(0.17961362552795712846e0) * t4669 * t8975 * t848 - F::new(0.17961362552795712846e0) * t41101 + F::new(0.11974241701863808564e0) * t5259 * t8975 * t833 + F::new(0.35922725105591425692e0) * t41106 + F::new(0.8980681276397856423e-1) * t41108 + F::new(0.47896966807455234256e0) * t35918 + F::new(0.66671395154821946448e-1) * t35922 + F::new(0.2666855806192877858e0) * t35926 + F::new(0.18183107769496894486e-1) * t35937 + t41115 - F::new(0.47896966807455234256e0) * t41116 * t8936 * t876 + F::new(0.47896966807455234256e0) * t41120 - F::new(0.35922725105591425692e0) * t4669 * t41122 * t321;
    t41126
}
