//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 503/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk503<F: Float>(t4295: F, t467: F, t488: F, t3906: F, t492: F, t500: F, t470: F, t3777: F, t498: F, t493: F, t1492: F, t1496: F, t486: F, t3913: F, t41: F, t1483: F, t1493: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4296 = t4295 * t467;
    let t4297 = t4296 * sigma0;
    let t4298 = t4297 * t488;
    let t4300 = t3906 * t467;
    let t4301 = t4300 * t492;
    let t4302 = t4301 * t500;
    let t4304 = 1.0 / t470;
    let t4305 = t4304 * t3777;
    let t4306 = t498 * t4305;
    let t4307 = t493 * t4306;
    let t4309 = t1492 * t1496;
    let t4310 = t486 * t4309;
    let t4312 = t3913 * t41;
    let t4313 = t4312 * t470;
    let t4314 = t486 * t4313;
    let t4316 = t1483 * t1493;
    (t4297, t4298, t4300, t4301, t4302, t4304, t4305, t4306, t4307, t4309, t4310, t4312, t4313, t4314, t4316)
}
