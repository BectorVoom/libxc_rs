//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2870/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2870<F: Float>(t23359: F, t822: F, t18632: F, t18681: F, t40318: F, t4366: F, t4504: F, t51522: F, t51538: F, t51547: F, t62866: F, t62872: F, t62874: F, t62881: F, t76169: F, t820: F, t837: F) -> F {
    let t77225 = t822 * t23359;
    let t77229 = F::cast_from(0.79025390195226139182e1_f64) * t4504 * t18681 * t18632 + F::cast_from(0.11044544084478153697e-3_f64) * t40318 + t51522 + F::cast_from(0.11708928647259339623e0_f64) * t62866 + F::cast_from(0.92196288561097162379e1_f64) * t4504 * t76169 * t4366 - F::cast_from(0.58544643236296698113e-1_f64) * t62872 - F::cast_from(0.69394917116090352834e-2_f64) * t62874 - F::cast_from(0.58544643236296698113e-1_f64) * t62881 + t51538 + t51547 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t77225 * t837;
    t77229
}
