//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1221/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1221(t39485: f64, t39487: f64, t39492: f64, t39493: f64, t39494: f64, t41414: f64, t41415: f64, t41419: f64, t41423: f64, t43072: f64, t43076: f64, t43079: f64) -> f64 {
    let t44216 = 0.18688645832733990742e0_f64 * t39485 - t39487 - t39492 - t39493 - t39494 - 0.52396431978519890152e-1_f64 * t43072 + t41414 - t41415 + t41419 - 0.43663693315433241794e-2_f64 * t43076 + 0.46574606203128791246e-1_f64 * t43079 + t41423;
    t44216
}
