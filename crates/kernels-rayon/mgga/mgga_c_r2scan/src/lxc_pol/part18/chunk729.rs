//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 729/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk729(t1793: f64, t406: f64, t1416: f64, t661: f64, t2036: f64, t410: f64, t230: f64, t4885: f64, t1654: f64, t761: f64, t2049: f64, t597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5980 = t406 * t1793;
    let t5982 = t1416 * t661;
    let t5985 = 12.0_f64 * t410 * t2036;
    let t5986 = t4885 * t230;
    let t5998 = t1654 * t761;
    let t6001 = t597 * t2049;
    (t5980, t5982, t5985, t5986, t5998, t6001)
}
