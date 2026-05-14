//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1005/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1005<F: Float>(t1084: F, t11927: F, t1461: F, t291: F, t8709: F, t1971: F, t818: F, t8448: F, t9846: F, t34235: F, t34238: F, t34241: F, t34245: F, t34249: F, t34252: F, t34255: F, t34258: F, t34264: F) -> (F,) {
    let t34269 = t1084 * t1461 * t8709 * t291 * t11927;
    let t34274 = t1084 * t1971 * t8448 * t818 * t9846;
    let t34276 = 0.51491428373437201895e-6 * t34235 + 0.20010856351627032588e-8 * t34238 + 0.17376185052903442709e-3 * t34241 + 0.24581606547037760418e-8 * t34245 - 0.81938688490125868062e-9 * t34249 - 0.51491428373437201896e-5 * t34252 - 0.16387737698025173612e-8 * t34255 + 0.11049275749843950005e-7 * t34258 + 0.66295654499063700028e-7 * t34264 - 0.54785992259642918774e-7 * t34269 + 0.39291224566445086216e-8 * t34274;
    (t34276,)
}
