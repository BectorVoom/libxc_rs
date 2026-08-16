//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1863/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1863(t14228: f64, t4342: f64, t3071: f64, t1025: f64, t10403: f64, t1041: f64, t10413: f64, t10909: f64, t10923: f64, t10927: f64, t14174: f64, t14180: f64, t14184: f64, t14189: f64, t14194: f64, t14198: f64, t14203: f64, t14207: f64, t14215: f64, t14222: f64, t2960: f64, t3070: f64, t3117: f64, t4590: f64, t4609: f64, t973: f64) -> (f64, f64, f64) {
    let t14229 = t4342 * t14228;
    let t14230 = t3071 * t14229;
    let t14233 = -5.0_f64 / 2304.0_f64 * t1041 * t14174 + 5.0_f64 / 6912.0_f64 * t3117 * t4590 + 5.0_f64 / 6912.0_f64 * t1041 * t14180 + 5.0_f64 / 13824.0_f64 * t1041 * t14184 + 5.0_f64 / 5184.0_f64 * t1041 * t14189 + t14194 - t2960 * t4609 / 54.0_f64 + t973 * t14198 / 288.0_f64 - t14203 / 20736.0_f64 + t14207 * t1025 / 1536.0_f64 + t10909 / 4608.0_f64 + t10403 * t14215 / 1152.0_f64 - t10413 * t14222 / 2304.0_f64 - t10923 / 648.0_f64 - t10927 / 162.0_f64 - t3070 * t14230 / 1152.0_f64;
    (t14229, t14230, t14233)
}
