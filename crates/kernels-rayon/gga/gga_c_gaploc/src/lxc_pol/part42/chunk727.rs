//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 727/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk727(t14425: f64, t14439: f64, t1052: f64, t12277: f64, t13334: f64, t13342: f64, t13349: f64, t13569: f64, t13577: f64, t13580: f64, t13584: f64, t14290: f64, t14294: f64, t331: f64) -> (f64, f64) {
    let t14440 = t14425 + t14439;
    let t14442 = -2.0_f64 * t1052 * t12277 + t14440 * t331 + t13334 - t13342 + t13349 - t13569 + t13577 - t13580 - t13584 - t14290 + t14294;
    (t14440, t14442)
}
