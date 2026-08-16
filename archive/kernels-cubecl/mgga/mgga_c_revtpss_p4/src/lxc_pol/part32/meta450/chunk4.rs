//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1636/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1636<F: Float>(t22452: F, t686: F, t9680: F, t10160: F, t10163: F, t10166: F, t1424: F, t14280: F, t14290: F, t14294: F, t14297: F, t213: F, t22433: F, t22441: F, t22447: F, t22450: F, t4071: F, t561: F, t6919: F) -> (F, F) {
    let t22453 = t22452 * t686;
    let t22454 = t9680 * t22453;
    let t22459 = -F::cast_from(0.26019841438354088051e-1_f64) * t14280 - F::cast_from(0.39512695097613069591e1_f64) * t1424 * t22433 - F::cast_from(0.65854491829355115987e0_f64) * t4071 * t6919 - F::cast_from(0.73171657588172351096e-2_f64) * t10160 + F::cast_from(0.65049603595885220126e-3_f64) * t10163 + F::cast_from(0.11565819519348392139e-2_f64) * t10166 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t22441 * t561 - F::cast_from(0.54878743191129263322e-2_f64) * t22447 - F::cast_from(0.10975748638225852664e-1_f64) * t22450 + F::cast_from(0.19514881078765566037e-1_f64) * t22454 - F::cast_from(0.14634331517634470219e-1_f64) * t14290 + F::cast_from(0.23131639038696784278e-2_f64) * t14294 + F::cast_from(0.13009920719177044025e-2_f64) * t14297;
    (t22453, t22459)
}
