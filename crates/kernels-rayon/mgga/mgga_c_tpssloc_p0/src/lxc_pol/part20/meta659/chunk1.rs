//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2453/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2453(t13961: f64, t3109: f64, t10263: f64, t10321: f64, t10403: f64, t10408: f64, t14122: f64, t1616: f64, t3070: f64, t3071: f64, t3132: f64, t42505: f64, t42541: f64, t43200: f64, t43206: f64, t43214: f64, t43219: f64, t43221: f64, t43226: f64, t43241: f64, t4337: f64, t4347: f64, t4609: f64) -> f64 {
    let t50229 = t3109 * t13961;
    let t50237 = t42541 * t14122 / 768.0_f64 - t43200 / 3456.0_f64 + t10403 * t3071 * t4347 * t3132 / 768.0_f64 - t43206 / 1152.0_f64 + t43214 / 648.0_f64 + t43219 / 3456.0_f64 + t43221 / 432.0_f64 + t43226 / 2304.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10408 * t4337 * t43241 + 11.0_f64 / 108.0_f64 * t10263 * t4609 - t50229 / 144.0_f64 - t42505 * t14122 / 144.0_f64 + t3070 * t3071 * t1616 * t10321 / 4608.0_f64;
    t50237
}
