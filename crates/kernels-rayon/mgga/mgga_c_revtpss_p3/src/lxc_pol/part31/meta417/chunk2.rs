//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1490/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1490(t233: f64, t6041: f64, t869: f64, t689: f64, t251: f64, t6016: f64, t822: f64, t6022: f64, t72: f64, t686: f64, t10530: f64, t10645: f64, t10647: f64, t10651: f64, t14558: f64, t14564: f64, t14570: f64, t18616: f64, t18632: f64, t18657: f64, t213: f64, t234: f64, t2815: f64, t4424: f64, t4494: f64, t4504: f64, t4514: f64, t4526: f64, t6017: f64, t820: f64, t837: f64, t879: f64) -> (f64, f64) {
    let t18688 = t233 * t6041;
    let t18689 = t869 * t18688;
    let t18690 = t689 * t18689;
    let t18699 = t251 * t6016;
    let t18714 = t822 * t6041;
    let t18718 = t6022 * t72;
    let t18719 = t18718 * t686;
    let t18720 = t10530 * t18719;
    let t18722 = -0.54878743191129263322e-2_f64 * t18690 + 0.65854491829355115987e0_f64 * t213 * t234 * t18657 - 0.65854491829355115987e0_f64 * t820 * t2815 * t6017 - 0.13009920719177044025e-2_f64 * t14558 - 0.65854491829355115987e0_f64 * t4514 * t18699 * t837 - 0.13170898365871023197e1_f64 * t820 * t4526 * t4424 + 0.26341796731742046394e1_f64 * t4504 * t4494 * t18632 + 0.26019841438354088051e-1_f64 * t14564 - t10645 - 0.13009920719177044025e-1_f64 * t10647 + t10651 - 0.65854491829355115987e0_f64 * t820 * t879 * t18616 - t14570 - 0.65854491829355115987e0_f64 * t820 * t18714 * t837 + 0.19514881078765566037e-1_f64 * t18720;
    (t18699, t18722)
}
