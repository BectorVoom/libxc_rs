//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3176/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3176<F: Float>(t56183: F, t56185: F, t56187: F, t56189: F, t56194: F, t56198: F, t56203: F, t56207: F, t56209: F, t56212: F, t56214: F, t56216: F) -> F {
    let t58531 = F::cast_from(0.79724444444444444445e0_f64) * t56183 - F::cast_from(0.11958666666666666667e1_f64) * t56185 - F::cast_from(0.59793333333333333333e0_f64) * t56187 - F::cast_from(0.17938e1_f64) * t56189 - F::cast_from(0.59793333333333333333e0_f64) * t56194 - F::cast_from(0.59793333333333333333e0_f64) * t56198 - F::cast_from(0.35876000000000000001e1_f64) * t56203 - F::cast_from(0.19931111111111111111e0_f64) * t56207 + F::cast_from(0.39862222222222222222e0_f64) * t56209 + F::cast_from(0.19931111111111111112e0_f64) * t56212 + F::cast_from(0.11958666666666666667e1_f64) * t56214 - F::cast_from(0.33218518518518518519e0_f64) * t56216;
    t58531
}
