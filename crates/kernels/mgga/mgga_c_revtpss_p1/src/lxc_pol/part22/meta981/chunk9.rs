//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3320/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3320<F: Float>(t40938: F, t4494: F, t4504: F, t4514: F, t51529: F, t51621: F, t51623: F, t51628: F, t51632: F, t51635: F, t51637: F, t51642: F, t62641: F, t62983: F, t62987: F, t62992: F, t62999: F, t837: F) -> F {
    let t63002 = -F::cast_from(0.39029762157531132076e-1_f64) * t51621 - F::cast_from(0.19514881078765566038e-1_f64) * t51623 - F::cast_from(0.13170898365871023197e1_f64) * t4514 * t62641 * t837 - F::cast_from(0.21951497276451705328e-1_f64) * t51628 + F::cast_from(0.15805078039045227837e2_f64) * t4504 * t4494 * t51529 + F::cast_from(0.26019841438354088049e-1_f64) * t62983 + F::cast_from(0.78059524315062264149e-1_f64) * t62987 + F::cast_from(0.78059524315062264152e-1_f64) * t51632 + F::cast_from(0.39029762157531132075e-1_f64) * t62992 + F::cast_from(0.92526556154787137113e-2_f64) * t51635 + F::cast_from(0.2601984143835408805e-2_f64) * t51637 - F::cast_from(0.65049603595885220126e-3_f64) * t40938 - F::cast_from(0.11565819519348392139e-2_f64) * t62999 + F::cast_from(0.10975748638225852664e-1_f64) * t51642;
    t63002
}
