//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 972/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk972(t14340: f64, t1506: f64, t14302: f64, t14305: f64, t14308: f64, t14310: f64, t14313: f64, t14316: f64, t14318: f64, t14322: f64, t14324: f64, t14326: f64, t14328: f64, t14331: f64, t14335: f64, t14338: f64) -> (f64, f64) {
    let t14341 = t14340 * t1506;
    let t14343 = t14302 / 192.0_f64 + 2.0_f64 / 3.0_f64 * t14305 - t14308 / 8.0_f64 - t14310 / 8.0_f64 + t14313 / 64.0_f64 - t14316 + t14318 / 12.0_f64 + 3.0_f64 / 8.0_f64 * t14322 + t14324 / 6.0_f64 - t14326 / 24.0_f64 - 3.0_f64 / 16.0_f64 * t14328 - t14331 / 16.0_f64 - t14335 / 16.0_f64 + t14338 / 8.0_f64 + 3.0_f64 / 256.0_f64 * t14341;
    (t14341, t14343)
}
