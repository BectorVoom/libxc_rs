//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 945/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk945<F: Float>(t10265: F, t1063: F, t10241: F, t6508: F, t6507: F, t1358: F, t2299: F, t3394: F, t488: F, t2339: F, t7888: F, t10232: F, t10236: F, t10238: F, t10240: F, t10245: F, t10248: F, t10251: F, t10255: F, t10259: F, t10261: F, t10264: F) -> (F, F, F, F, F) {
    let t10267 = F::new(0.85365019907028448797e-1) * t1063 * t10265;
    let t10268 = t6508 * t10241;
    let t10269 = t6507 * t10268;
    let t10271 = F::new(0.63233348079280332442e-2) * t1358 * t10269;
    let t10272 = t2299 * t3394;
    let t10273 = t10272 * t488;
    let t10275 = F::new(0.31616674039640166221e-2) * t1358 * t10273;
    let t10276 = t7888 * t2339;
    let t10278 = F::new(0.94850022118920498663e-2) * t1358 * t10276;
    let t10279 = F::new(0.31616674039640166221e-2) * t1358 * t10232 + t10236 - t10238 - t10240 + t10245 - t10248 + t10251 - t10255 - t10259 + t10261 + t10264 + t10267 - t10271 - t10275 + t10278;
    (t10268, t10269, t10273, t10276, t10279)
}
