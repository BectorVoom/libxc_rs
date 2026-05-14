//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 471/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk471<F: Float>(t4179: F, t498: F, t1818: F, t195: F, t1835: F, t500: F, t1022: F, t1532: F, t1819: F, t1911: F, t4183: F, t4214: F, t4220: F, t4232: F, t4252: F, t4255: F, t4259: F, t4336: F, t4338: F, t4351: F, t5407: F, t5409: F, t5998: F, t6000: F, t6001: F, t6002: F, t6003: F) -> (F,) {
    let t6284 = t4179 * t498;
    let t6287 = t195 * t1818;
    let t6290 = t500 * t1835;
    let t6293 = t4336 - t4338 - 0.31091e-1 * t1911 * t1532 + t5998 - t6000 + 0.62182e-1 * t1819 * t6284 - 0.93273e-1 * t6287 * t4183 + t4214 - t4220 - t6001 - t5407 - t5409 + t6002 + t6003 + 0.93273e-1 * t1022 * t6290 + t4232 + t4252 - t4255 - t4259 - t4351;
    (t6293,)
}
