//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 600/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk600<F: Float>(t4208: F, t3786: F, t470: F, t487: F, t1413: F, t1481: F, t1489: F, t1501: F, t1513: F, t1517: F, t4176: F, t4183: F, t4186: F, t4190: F, t4194: F, t4198: F, t4201: F, t4206: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t4209 = t4208 * sigma0;
    let t4210 = t470 * t3786;
    let t4211 = t487 * t4210;
    let t4212 = t4209 * t4211;
    let t4214 = t1481 * t1413;
    let t4215 = t4214 * sigma0;
    let t4216 = t4215 * t1489;
    let t4218 = t1501 * t1513;
    let t4220 = t1501 * t1517;
    let t4222 = t4176 / 24.0 - 19.0 / 144.0 * t4183 + t4186 / 18.0 + t4190 / 256.0 - t4194 / 192.0 - t4198 / 16.0 + t4201 / 3.0 - t4206 / 12.0 + t4212 / 8.0 - t4216 / 8.0 + t4218 / 24.0 - t4220 / 96.0;
    (t4211, t4212, t4214, t4215, t4216, t4218, t4220, t4222)
}
