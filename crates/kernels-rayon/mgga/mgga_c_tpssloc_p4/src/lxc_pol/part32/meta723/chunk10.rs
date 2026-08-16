//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2316/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2316(t27604: f64, t4993: f64, t19095: f64, t24733: f64, t1207: f64, t19024: f64, t7337: f64, t19046: f64, t7338: f64, t6169: f64, t7344: f64, t1218: f64, t1232: f64, t1737: f64, t1748: f64, t18307: f64, t18943: f64, t18959: f64, t24716: f64, t6221: f64, t7339: f64, t7345: f64, t86164: f64, t95242: f64, t95244: f64, t95276: f64, t95440: f64) -> f64 {
    let t104007 = t27604 * t4993;
    let t104009 = t24733 * t19095;
    let t104012 = t1207 * t7337 * t19024;
    let t104015 = t19046 * t7338;
    let t104018 = t6169 * t7344;
    let t104029 = t95242 - t95244 + t24716 * t6221 / 1536.0_f64 + t7339 * t18943 / 1536.0_f64 + t104007 / 324.0_f64 - t104009 / 2304.0_f64 + 19.0_f64 / 864.0_f64 * t104012 * t1218 + t104015 * t1218 / 1536.0_f64 - t104018 * t1232 / 2304.0_f64 - t95276 * t1748 / 1152.0_f64 - t95440 * t1737 / 144.0_f64 - t86164 * t18307 / 256.0_f64 - t7345 * t18959 / 1152.0_f64;
    t104029
}
