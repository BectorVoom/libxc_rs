//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2126/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2126<F: Float>(t27989: F, t94802: F, t25899: F, t98303: F, t1444: F, t1904: F, t25924: F, t26079: F, t26081: F, t27837: F, t27909: F, t28002: F, t4003: F, t4132: F, t7295: F, t7298: F, t94906: F, t94909: F, t94911: F, t94914: F, t94917: F, t94919: F, t94922: F, t94931: F, t97909: F, t98050: F) -> F {
    let t98390 = F::cast_from(0.25702851531048074406e-1_f64) * t94802 * t27989;
    let t98399 = F::cast_from(0.25702851531048074406e-1_f64) * t25899 * t98303;
    let t98414 = t98390 + F::cast_from(0.25702851531048074406e-1_f64) * t94909 + F::cast_from(0.12851425765524037203e-1_f64) * t94911 + F::cast_from(0.34270468708064099208e-2_f64) * t94914 - F::cast_from(0.65854491829355115987e0_f64) * t94906 * t1904 + t94917 - F::cast_from(0.48186823267806663678e-3_f64) * t94919 - F::cast_from(0.14456046980341999104e-1_f64) * t94922 + t98399 - F::cast_from(0.65854491829355115987e0_f64) * t27909 * t4132 - t94931 + F::cast_from(0.17347256376410398924e1_f64) * t98050 * t7298 - F::cast_from(0.52041769129231196772e1_f64) * t7295 * t25924 * t28002 * t1444 - F::cast_from(0.8673628188205199462e0_f64) * t7295 * t26079 * t97909 * t4003 - F::cast_from(0.8673628188205199462e0_f64) * t27837 * t26081;
    t98414
}
