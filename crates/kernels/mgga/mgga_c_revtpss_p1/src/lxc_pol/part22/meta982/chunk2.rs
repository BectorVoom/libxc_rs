//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3325/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3325<F: Float>(t15011: F, t15030: F, t2770: F, t2828: F, t41095: F, t41098: F, t41102: F, t41105: F, t4474: F, t4534: F, t51268: F, t51277: F, t51726: F, t51729: F, t51731: F, t51733: F, t51739: F, t51741: F, t6071: F, t63058: F, t63062: F, t63064: F, t63085: F, t865: F) -> F {
    let t63088 = -F::cast_from(0.46263278077393568556e-2_f64) * t51268 - t41095 - F::cast_from(0.23131639038696784277e-2_f64) * t63058 - F::cast_from(0.19514881078765566038e-1_f64) * t63062 - F::cast_from(0.39029762157531132074e-1_f64) * t63064 + F::cast_from(0.13170898365871023197e1_f64) * t865 * t2770 * t6071 * t2828 - F::cast_from(0.39029762157531132076e-1_f64) * t51277 + F::cast_from(0.92526556154787137112e-2_f64) * t41098 - F::cast_from(0.60712963356159538784e-1_f64) * t41102 + F::cast_from(0.39274398764404314548e-3_f64) * t41105 - F::cast_from(0.46263278077393568556e-2_f64) * t51726 - F::cast_from(0.43902994552903410656e-1_f64) * t51729 + F::cast_from(0.52683593463484092788e1_f64) * t4474 * t15030 - F::cast_from(0.19514881078765566038e-1_f64) * t51731 + F::cast_from(0.520396828767081761e-2_f64) * t51733 - F::cast_from(0.19514881078765566038e-1_f64) * t51739 - F::cast_from(0.26341796731742046394e1_f64) * t15011 * t4534 - F::cast_from(0.19514881078765566038e-1_f64) * t63085 + F::cast_from(0.29268663035268940438e-1_f64) * t51741;
    t63088
}
