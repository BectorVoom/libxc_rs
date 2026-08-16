//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2792/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2792<F: Float>(t213: F, t225: F, t40321: F, t10872: F, t14502: F, t14546: F, t14972: F, t2646: F, t39612: F, t39617: F, t39622: F, t4494: F, t4504: F, t4514: F, t50666: F, t50758: F, t50916: F, t51299: F, t51306: F, t820: F, t837: F, t879: F) -> F {
    let t51320 = t213 * t225 * t40321;
    let t51327 = -t51299 - F::cast_from(0.29272321618148349057e-1_f64) * t39612 - F::cast_from(0.9757440539382783019e-2_f64) * t39617 + F::cast_from(0.16463622957338778996e-1_f64) * t39622 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t14502 * t2646 - F::cast_from(0.19756347548806534796e1_f64) * t4514 * t51306 * t837 + F::cast_from(0.13170898365871023197e1_f64) * t4504 * t4494 * t50666 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t879 * t50916 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t14972 * t2646 + F::cast_from(0.15805078039045227836e2_f64) * t51320 * t4494 * t50758 - F::cast_from(0.23707617058567841754e2_f64) * t14546 * t4494 * t10872;
    t51327
}
