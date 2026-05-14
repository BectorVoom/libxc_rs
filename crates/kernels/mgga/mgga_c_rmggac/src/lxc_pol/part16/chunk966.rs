//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 966/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk966<F: Float>(t43752: F, t43757: F, t43761: F, t43763: F, t47345: F, t47347: F, t47349: F, t47351: F, t47353: F, t47355: F, t47357: F, t47359: F, t47361: F, t47365: F, t47367: F, t47371: F, t47375: F, t4985: F, t9624: F) -> (F,) {
    let t49006 = -t43752 + 0.1702583995731913576e-4 * t47345 - 0.5107751987195740728e-4 * t47347 + 0.5107751987195740728e-4 * t47349 + 0.3405167991463827152e-4 * t47351 - 0.1702583995731913576e-4 * t47353 + 0.10215503974391481456e-3 * t47355 - 0.15323255961587222184e-3 * t47357 - 0.11918087970123395032e-3 * t47359 - 0.68186654135613354325e-2 * t47361 - 0.68186654135613354325e-2 * t47365 + 0.20455996240684006298e-1 * t47367 - t43757 - t43761 - t43763 - 0.5987120850931904282e-1 * t47371 + 0.39726959900411316773e-4 * t47375 - 0.23948483403727617128e0 * t4985 * t9624;
    (t49006,)
}
