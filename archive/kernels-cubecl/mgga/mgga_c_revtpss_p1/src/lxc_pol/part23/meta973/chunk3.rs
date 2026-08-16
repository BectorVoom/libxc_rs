//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3301/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3301<F: Float>(t1399: F, t14193: F, t14224: F, t22005: F, t22009: F, t47444: F, t5675: F, t5745: F, t5755: F, t75269: F, t75274: F, t85580: F, t86445: F, t86506: F, t86634: F, t86639: F, t86643: F, t86647: F) -> F {
    let t86649 = -F::cast_from(0.32927245914677557992e-1_f64) * t75269 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t22009 * t14224 + F::cast_from(0.21951497276451705328e-1_f64) * t75274 + F::cast_from(0.92196288561097162379e1_f64) * t5745 * t86445 * t5675 - F::cast_from(0.19756347548806534796e1_f64) * t5755 * t86506 * t1399 - F::cast_from(0.11853808529283920877e2_f64) * t14193 * t22005 * t85580 - F::cast_from(0.32927245914677557992e-1_f64) * t86634 + F::cast_from(0.30356481678079769392e-1_f64) * t47444 - F::cast_from(0.9757440539382783019e-2_f64) * t86639 + F::cast_from(0.16463622957338778997e-1_f64) * t86643 - F::cast_from(0.32927245914677557992e-1_f64) * t86647;
    t86649
}
