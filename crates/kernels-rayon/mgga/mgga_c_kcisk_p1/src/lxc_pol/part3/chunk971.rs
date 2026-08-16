//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 971/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk971(t4197: f64, t4215: f64, t1512: f64, t4188: f64, t1504: f64, t13288: f64, t470: f64, t487: f64, t1487: f64, t4236: f64, t4235: f64, t13382: f64, t492: f64) -> (f64, f64, f64, f64, f64) {
    let t14328 = t4215 * t4197;
    let t14330 = t1512 * t4188;
    let t14331 = t1504 * t14330;
    let t14333 = t470 * t13288;
    let t14334 = t487 * t14333;
    let t14335 = t1487 * t14334;
    let t14337 = t1512 * t4236;
    let t14338 = t4235 * t14337;
    let t14340 = t13382 * t492;
    (t14328, t14331, t14335, t14338, t14340)
}
