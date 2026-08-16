//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3320/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3320(t40938: f64, t4494: f64, t4504: f64, t4514: f64, t51529: f64, t51621: f64, t51623: f64, t51628: f64, t51632: f64, t51635: f64, t51637: f64, t51642: f64, t62641: f64, t62983: f64, t62987: f64, t62992: f64, t62999: f64, t837: f64) -> f64 {
    let t63002 = -0.39029762157531132076e-1_f64 * t51621 - 0.19514881078765566038e-1_f64 * t51623 - 0.13170898365871023197e1_f64 * t4514 * t62641 * t837 - 0.21951497276451705328e-1_f64 * t51628 + 0.15805078039045227837e2_f64 * t4504 * t4494 * t51529 + 0.26019841438354088049e-1_f64 * t62983 + 0.78059524315062264149e-1_f64 * t62987 + 0.78059524315062264152e-1_f64 * t51632 + 0.39029762157531132075e-1_f64 * t62992 + 0.92526556154787137113e-2_f64 * t51635 + 0.2601984143835408805e-2_f64 * t51637 - 0.65049603595885220126e-3_f64 * t40938 - 0.11565819519348392139e-2_f64 * t62999 + 0.10975748638225852664e-1_f64 * t51642;
    t63002
}
