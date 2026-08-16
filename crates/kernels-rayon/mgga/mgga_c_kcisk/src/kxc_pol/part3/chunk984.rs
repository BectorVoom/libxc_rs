//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 984/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk984(t14443: f64, t14510: f64, t467: f64, t488: f64, t1501: f64, t4306: f64, t13320: f64, t4231: f64, t4230: f64, t13394: f64, t6317: f64, t6316: f64, sigma0: f64) -> (f64, f64, f64, f64) {
    let t14511 = t14443 + t14510;
    let t14512 = t14511 * t467;
    let t14513 = t14512 * sigma0;
    let t14514 = t14513 * t488;
    let t14516 = t1501 * t4306;
    let t14518 = t4231 * t13320;
    let t14519 = t4230 * t14518;
    let t14521 = t6317 * t13394;
    let t14522 = t6316 * t14521;
    (t14514, t14516, t14519, t14522)
}
