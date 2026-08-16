//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2457/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2457(t10903: f64, t14507: f64, t14651: f64, t3069: f64, t10394: f64, t10403: f64, t10937: f64, t10952: f64, t13995: f64, t14069: f64, t14077: f64, t14103: f64, t14211: f64, t1622: f64, t3071: f64, t3073: f64, t3120: f64, t3123: f64, t3134: f64, t42746: f64, t43262: f64, t43273: f64, t43277: f64, t43281: f64, t43285: f64, t607: f64, t883: f64) -> f64 {
    let t50302 = t14507 * t10903;
    let t50324 = t14651 * t3069;
    let t50329 = -t50302 * t3134 / 96.0_f64 + t42746 * t1622 / 4608.0_f64 + t10403 * t3071 * t14211 * t3120 * t883 * t607 / 768.0_f64 - 209.0_f64 / 3888.0_f64 * t43262 + t43273 / 144.0_f64 + t43277 / 768.0_f64 - t43281 / 768.0_f64 + t43285 / 4608.0_f64 - t14077 * t3123 / 192.0_f64 - t10952 * t14103 / 1024.0_f64 - t10937 * t14069 / 144.0_f64 + t50324 * t3073 / 768.0_f64 + t13995 * t10394 / 1536.0_f64;
    t50329
}
