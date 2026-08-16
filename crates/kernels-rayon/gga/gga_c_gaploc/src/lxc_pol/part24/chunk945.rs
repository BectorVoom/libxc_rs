//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 945/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk945(t10265: f64, t1063: f64, t10241: f64, t6508: f64, t6507: f64, t1358: f64, t2299: f64, t3394: f64, t488: f64, t2339: f64, t7888: f64, t10232: f64, t10236: f64, t10238: f64, t10240: f64, t10245: f64, t10248: f64, t10251: f64, t10255: f64, t10259: f64, t10261: f64, t10264: f64) -> (f64, f64, f64, f64, f64) {
    let t10267 = 0.85365019907028448797e-1_f64 * t1063 * t10265;
    let t10268 = t6508 * t10241;
    let t10269 = t6507 * t10268;
    let t10271 = 0.63233348079280332442e-2_f64 * t1358 * t10269;
    let t10272 = t2299 * t3394;
    let t10273 = t10272 * t488;
    let t10275 = 0.31616674039640166221e-2_f64 * t1358 * t10273;
    let t10276 = t7888 * t2339;
    let t10278 = 0.94850022118920498663e-2_f64 * t1358 * t10276;
    let t10279 = 0.31616674039640166221e-2_f64 * t1358 * t10232 + t10236 - t10238 - t10240 + t10245 - t10248 + t10251 - t10255 - t10259 + t10261 + t10264 + t10267 - t10271 - t10275 + t10278;
    (t10268, t10269, t10273, t10276, t10279)
}
