//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1194/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1194(t10894: f64, t3086: f64, t30285: f64, t3332: f64, t6165: f64, t11646: f64, t25983: f64, t11649: f64, t30792: f64, t12529: f64, t6395: f64, t3281: f64, t9273: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43266 = t10894 * t3086;
    let t43269 = t6165 * t3332 * t30285;
    let t43271 = t25983 * t11646;
    let t43273 = t30792 * t11649;
    let t43275 = t6395 * t12529;
    let t43277 = t3281 * t9273;
    (t43266, t43269, t43271, t43273, t43275, t43277)
}
