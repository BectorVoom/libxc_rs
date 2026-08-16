//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2684/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684(t74699: f64, t74735: f64, t74754: f64, t74765: f64, t225: f64, t1307: f64, t16305: f64, t16311: f64, t19876: f64, t19890: f64, t19966: f64, t40124: f64, t40145: f64, t5246: f64, t54534: f64, t554: f64, t559: f64, t57127: f64, t57143: f64, t57145: f64, t57158: f64, t57160: f64, t57170: f64, t6414: f64, t74677: f64) -> (f64, f64, f64) {
    let t74767 = t74699 + t74735 + t74754 + t74765;
    let t74768 = t74767 * t225;
    let t74786 = -t5246 * t16305 * t16311 * t6414 * t1307 / 128.0_f64 - t19876 * t19890 / 64.0_f64 + t74768 * t554 * t559 / 3072.0_f64 + 595.0_f64 / 10368.0_f64 * t40124 - 595.0_f64 / 10368.0_f64 * t40145 - t54534 + 35.0_f64 / 192.0_f64 * t57127 + t19876 * t19966 / 512.0_f64 + 35.0_f64 / 384.0_f64 * t57143 - 7.0_f64 / 384.0_f64 * t57145 + 7.0_f64 / 4.0_f64 * t57158 - 7.0_f64 / 8.0_f64 * t57160 - 7.0_f64 / 16.0_f64 * t57170 - t5246 * t16305 * t16311 * t74677 / 64.0_f64;
    (t74767, t74768, t74786)
}
