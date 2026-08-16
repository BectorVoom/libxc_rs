//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1672/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1672(t12345: f64, t1369: f64, t12215: f64, t12317: f64, t12320: f64, t12323: f64, t12325: f64, t12330: f64, t12331: f64, t12335: f64, t12336: f64, t12340: f64, t3783: f64, t3876: f64, t559: f64) -> (f64, f64) {
    let t12346 = t12345 * t1369;
    let t12348 = -7.0_f64 / 16.0_f64 * t12317 - t12215 * t12320 / 4.0_f64 - 7.0_f64 / 1536.0_f64 * t12323 + 119.0_f64 / 4608.0_f64 * t12325 - t12330 + t12331 * t559 / 3072.0_f64 - t12335 - t12336 * t1369 / 256.0_f64 + 7.0_f64 / 192.0_f64 * t12340 - t3783 * t3876 / 256.0_f64 - 119.0_f64 / 1152.0_f64 * t12346;
    (t12346, t12348)
}
