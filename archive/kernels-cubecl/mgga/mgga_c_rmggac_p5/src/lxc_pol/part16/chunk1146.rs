//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1146/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1146<F: Float>(t1356: F, t1550: F, t1624: F, t1923: F, t1953: F, t2231: F, t2471: F, t38140: F, t42132: F, t44382: F, t44385: F, t47933: F, t47935: F, t47946: F, t47948: F, t47952: F, t47957: F, t47961: F, t47963: F, t49432: F, t6344: F, t702: F, t72: F, t8188: F) -> F {
    let t49770 = -F::cast_from(0.35922725105591425692e0_f64) * t47933 - F::cast_from(0.23948483403727617128e0_f64) * t47935 + F::cast_from(0.1454648621559751559e0_f64) * t42132 - t44382 - t44385 + F::cast_from(0.39914139006212695214e-1_f64) * t1356 * t49432 - F::cast_from(0.5107751987195740728e-4_f64) * t47946 - F::cast_from(0.5107751987195740728e-4_f64) * t47948 + F::cast_from(0.5107751987195740728e-4_f64) * t47952 - t38140 - F::cast_from(0.23948483403727617128e0_f64) * t1550 * t2471 * t1624 - F::cast_from(0.2363e1_f64) * t1923 * t8188 + F::cast_from(0.85129199786595678799e-5_f64) * t47957 - F::cast_from(0.2553875993597870364e-4_f64) * t47961 + F::cast_from(0.5107751987195740728e-4_f64) * t47963 + t72 * t1953 * t2231 + t72 * t6344 * t702;
    t49770
}
