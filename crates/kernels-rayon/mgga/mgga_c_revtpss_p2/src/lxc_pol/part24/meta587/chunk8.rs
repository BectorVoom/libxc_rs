//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1832/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1832(t6918: f64, t1424: f64, t4076: f64, t47561: f64, t47920: f64, t47932: f64, t47938: f64, t49468: f64, t49474: f64, t74733: f64, t74757: f64, t74770: f64, t86300: f64, t86311: f64, t86314: f64, t86317: f64, t86346: f64) -> f64 {
    let t92259 = t6918 * t6918;
    let t92267 = -0.18505311230957427423e-1_f64 * t47920 + 0.21951497276451705328e-1_f64 * t86300 - 0.13878983423218070567e-1_f64 * t74733 + 0.18505311230957427422e-1_f64 * t47932 + 0.1040793657534163522e-1_f64 * t47938 + t47561 - 0.68293547082294194357e-1_f64 * t49468 - 0.7805952431506226415e-2_f64 * t74757 + 0.39029762157531132076e-1_f64 * t86311 + 0.13170898365871023197e0_f64 * t86314 + 0.39512695097613069591e1_f64 * t1424 * t4076 * t92259 - 0.13170898365871023197e0_f64 * t86317 - 0.44178176337912614788e-3_f64 * t49474 + 0.13878983423218070567e-1_f64 * t74770 + 0.65854491829355115985e-1_f64 * t86346;
    t92267
}
