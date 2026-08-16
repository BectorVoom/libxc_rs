//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1002/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1002<F: Float>(t40734: F, t5259: F, t321: F, t333: F, t35873: F, t35877: F, t35886: F, t35890: F, t41015: F, t41077: F, t41079: F, t41084: F, t41086: F, t41088: F, t41091: F, t4669: F, t5148: F, t838: F) -> F {
    let t41095 = t5259 * t40734;
    let t41097 = F::cast_from(0.27274661654245341729e-1_f64) * t35873 - F::cast_from(0.20001418546446583934e0_f64) * t35877 + F::cast_from(0.18183107769496894486e0_f64) * t35886 + F::cast_from(0.54549323308490683458e-1_f64) * t35890 - F::cast_from(0.8980681276397856423e0_f64) * t41077 + F::cast_from(0.5987120850931904282e-1_f64) * t41079 - F::cast_from(0.23948483403727617128e0_f64) * t5148 * t41015 * t321 - F::cast_from(0.17961362552795712846e0_f64) * t41084 - F::cast_from(0.5987120850931904282e-1_f64) * t41086 + F::cast_from(0.23948483403727617128e0_f64) * t838 * t41088 - F::cast_from(0.35922725105591425692e0_f64) * t4669 * t41091 * t333 - F::cast_from(0.5987120850931904282e-1_f64) * t41095;
    t41097
}
