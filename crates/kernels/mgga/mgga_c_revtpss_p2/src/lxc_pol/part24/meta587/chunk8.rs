//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1832/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1832<F: Float>(t6918: F, t1424: F, t4076: F, t47561: F, t47920: F, t47932: F, t47938: F, t49468: F, t49474: F, t74733: F, t74757: F, t74770: F, t86300: F, t86311: F, t86314: F, t86317: F, t86346: F) -> F {
    let t92259 = t6918 * t6918;
    let t92267 = -F::cast_from(0.18505311230957427423e-1_f64) * t47920 + F::cast_from(0.21951497276451705328e-1_f64) * t86300 - F::cast_from(0.13878983423218070567e-1_f64) * t74733 + F::cast_from(0.18505311230957427422e-1_f64) * t47932 + F::cast_from(0.1040793657534163522e-1_f64) * t47938 + t47561 - F::cast_from(0.68293547082294194357e-1_f64) * t49468 - F::cast_from(0.7805952431506226415e-2_f64) * t74757 + F::cast_from(0.39029762157531132076e-1_f64) * t86311 + F::cast_from(0.13170898365871023197e0_f64) * t86314 + F::cast_from(0.39512695097613069591e1_f64) * t1424 * t4076 * t92259 - F::cast_from(0.13170898365871023197e0_f64) * t86317 - F::cast_from(0.44178176337912614788e-3_f64) * t49474 + F::cast_from(0.13878983423218070567e-1_f64) * t74770 + F::cast_from(0.65854491829355115985e-1_f64) * t86346;
    t92267
}
