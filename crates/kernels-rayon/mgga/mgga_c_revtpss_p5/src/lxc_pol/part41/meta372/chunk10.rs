//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1223/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1223(t6072: f64, t779: f64, t689: f64, t1580: f64, t4321: f64, t6042: f64, t786: f64, t789: f64, t6049: f64, t14987: f64, t4481: f64, t11040: f64, t15011: f64, t15062: f64, t15063: f64, t2765: f64, t4474: f64, t4487: f64, t4534: f64) -> f64 {
    let t18811 = t779 * t6072;
    let t18812 = t689 * t18811;
    let t18814 = t4321 * t1580;
    let t18815 = t689 * t18814;
    let t18821 = t786 * t6042;
    let t18822 = t18821 * t789;
    let t18825 = t779 * t6049;
    let t18826 = t689 * t18825;
    let t18828 = t14987 * t4481;
    let t18836 = 0.54878743191129263322e-2_f64 * t18812 + 0.10975748638225852664e-1_f64 * t18815 + 0.13170898365871023197e1_f64 * t2765 * t6049 + 0.26341796731742046394e1_f64 * t4474 * t4487 + t15062 + 0.9757440539382783019e-2_f64 * t18822 + 0.14634331517634470219e-1_f64 * t15063 - t11040 - 0.10975748638225852664e-1_f64 * t18826 - 0.19514881078765566037e-1_f64 * t18828 - 0.13170898365871023197e1_f64 * t15011 * t1580 - 0.13170898365871023197e1_f64 * t4474 * t4534 - 0.65854491829355115987e0_f64 * t2765 * t6072;
    t18836
}
