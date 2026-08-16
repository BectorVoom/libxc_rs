//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 520/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk520(t4209: f64, t4211: f64, t1413: f64, t1481: f64, t1489: f64, t1501: f64, t1513: f64, t1517: f64, t4176: f64, t4183: f64, t4186: f64, t4190: f64, t4194: f64, t4198: f64, t4201: f64, t4206: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4212 = t4209 * t4211;
    let t4214 = t1481 * t1413;
    let t4215 = t4214 * sigma0;
    let t4216 = t4215 * t1489;
    let t4218 = t1501 * t1513;
    let t4220 = t1501 * t1517;
    let t4222 = t4176 / 24.0_f64 - 19.0_f64 / 144.0_f64 * t4183 + t4186 / 18.0_f64 + t4190 / 256.0_f64 - t4194 / 192.0_f64 - t4198 / 16.0_f64 + t4201 / 3.0_f64 - t4206 / 12.0_f64 + t4212 / 8.0_f64 - t4216 / 8.0_f64 + t4218 / 24.0_f64 - t4220 / 96.0_f64;
    (t4212, t4214, t4215, t4216, t4218, t4220, t4222)
}
