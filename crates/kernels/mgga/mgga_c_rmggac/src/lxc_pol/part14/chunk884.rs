//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 884/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk884<F: Float>(t27091: F, t40901: F, t40487: F, t5148: F, t39059: F, t5271: F, t39063: F, t5259: F, t2402: F, t839: F, t2367: F, t321: F, t40734: F, t333: F, t35873: F, t35877: F, t35886: F, t35890: F, t41015: F, t4669: F, t838: F) -> (F, F, F) {
    let t41077 = t27091 * t40901;
    let t41079 = t5148 * t40487;
    let t41084 = t5271 * t39059;
    let t41086 = t5259 * t39063;
    let t41088 = t2402 * t839;
    let t41091 = t2367 * t321;
    let t41095 = t5259 * t40734;
    let t41097 = 0.27274661654245341729e-1 * t35873 - 0.20001418546446583934e0 * t35877 + 0.18183107769496894486e0 * t35886 + 0.54549323308490683458e-1 * t35890 - 0.8980681276397856423e0 * t41077 + 0.5987120850931904282e-1 * t41079 - 0.23948483403727617128e0 * t5148 * t41015 * t321 - 0.17961362552795712846e0 * t41084 - 0.5987120850931904282e-1 * t41086 + 0.23948483403727617128e0 * t838 * t41088 - 0.35922725105591425692e0 * t4669 * t41091 * t333 - 0.5987120850931904282e-1 * t41095;
    (t41088, t41091, t41097)
}
