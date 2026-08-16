//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3325/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3325(t15011: f64, t15030: f64, t2770: f64, t2828: f64, t41095: f64, t41098: f64, t41102: f64, t41105: f64, t4474: f64, t4534: f64, t51268: f64, t51277: f64, t51726: f64, t51729: f64, t51731: f64, t51733: f64, t51739: f64, t51741: f64, t6071: f64, t63058: f64, t63062: f64, t63064: f64, t63085: f64, t865: f64) -> f64 {
    let t63088 = -0.46263278077393568556e-2_f64 * t51268 - t41095 - 0.23131639038696784277e-2_f64 * t63058 - 0.19514881078765566038e-1_f64 * t63062 - 0.39029762157531132074e-1_f64 * t63064 + 0.13170898365871023197e1_f64 * t865 * t2770 * t6071 * t2828 - 0.39029762157531132076e-1_f64 * t51277 + 0.92526556154787137112e-2_f64 * t41098 - 0.60712963356159538784e-1_f64 * t41102 + 0.39274398764404314548e-3_f64 * t41105 - 0.46263278077393568556e-2_f64 * t51726 - 0.43902994552903410656e-1_f64 * t51729 + 0.52683593463484092788e1_f64 * t4474 * t15030 - 0.19514881078765566038e-1_f64 * t51731 + 0.520396828767081761e-2_f64 * t51733 - 0.19514881078765566038e-1_f64 * t51739 - 0.26341796731742046394e1_f64 * t15011 * t4534 - 0.19514881078765566038e-1_f64 * t63085 + 0.29268663035268940438e-1_f64 * t51741;
    t63088
}
