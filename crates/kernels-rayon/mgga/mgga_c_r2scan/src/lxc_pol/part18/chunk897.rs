//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 897/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk897(t2266: f64, t481: f64, t9589: f64, t2900: f64, t6621: f64, t806: f64, t35: f64, t990: f64, t1216: f64, t1248: f64, t2904: f64, t4911: f64) -> (f64, f64, f64, f64, f64) {
    let t9591 = t2266 * t9589 * t481;
    let t9592 = 3.0_f64 * t9591;
    let t9597 = t6621 * t2900;
    let t9598 = t9597 * t806;
    let t9601 = t990 * t35;
    let t9602 = t9601 * t1216;
    let t9607 = t1248 * t2904;
    let t9608 = t9607 * t806;
    let t9612 = -t1216 - 3.0_f64 * t4911;
    (t9592, t9598, t9602, t9608, t9612)
}
