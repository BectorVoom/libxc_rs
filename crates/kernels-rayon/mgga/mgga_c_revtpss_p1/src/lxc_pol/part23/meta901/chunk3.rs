//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2870/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2870(t23359: f64, t822: f64, t18632: f64, t18681: f64, t40318: f64, t4366: f64, t4504: f64, t51522: f64, t51538: f64, t51547: f64, t62866: f64, t62872: f64, t62874: f64, t62881: f64, t76169: f64, t820: f64, t837: f64) -> f64 {
    let t77225 = t822 * t23359;
    let t77229 = 0.79025390195226139182e1_f64 * t4504 * t18681 * t18632 + 0.11044544084478153697e-3_f64 * t40318 + t51522 + 0.11708928647259339623e0_f64 * t62866 + 0.92196288561097162379e1_f64 * t4504 * t76169 * t4366 - 0.58544643236296698113e-1_f64 * t62872 - 0.69394917116090352834e-2_f64 * t62874 - 0.58544643236296698113e-1_f64 * t62881 + t51538 + t51547 - 0.65854491829355115987e0_f64 * t820 * t77225 * t837;
    t77229
}
