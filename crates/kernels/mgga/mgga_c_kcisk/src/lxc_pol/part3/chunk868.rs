//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 868/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk868<F: Float>(t4197: F, t4215: F, t1512: F, t4188: F, t1504: F, t13288: F, t470: F, t487: F, t1487: F, t4236: F, t4235: F, t13382: F, t492: F, t1506: F, t14302: F, t14305: F, t14308: F, t14310: F, t14313: F, t14316: F, t14318: F, t14322: F, t14324: F, t14326: F) -> (F, F, F, F, F, F) {
    let t14328 = t4215 * t4197;
    let t14330 = t1512 * t4188;
    let t14331 = t1504 * t14330;
    let t14333 = t470 * t13288;
    let t14334 = t487 * t14333;
    let t14335 = t1487 * t14334;
    let t14337 = t1512 * t4236;
    let t14338 = t4235 * t14337;
    let t14340 = t13382 * t492;
    let t14341 = t14340 * t1506;
    let t14343 = t14302 / 192.0 + 2.0 / 3.0 * t14305 - t14308 / 8.0 - t14310 / 8.0 + t14313 / 64.0 - t14316 + t14318 / 12.0 + 3.0 / 8.0 * t14322 + t14324 / 6.0 - t14326 / 24.0 - 3.0 / 16.0 * t14328 - t14331 / 16.0 - t14335 / 16.0 + t14338 / 8.0 + 3.0 / 256.0 * t14341;
    (t14328, t14331, t14335, t14338, t14341, t14343)
}
