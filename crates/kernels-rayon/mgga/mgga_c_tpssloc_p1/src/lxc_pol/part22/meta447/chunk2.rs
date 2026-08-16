//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1803/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1803(t16225: f64, t16311: f64, t16305: f64, t1825: f64, t5308: f64, t16224: f64, t12286: f64, t1341: f64, t16239: f64, t16241: f64, t16269: f64, t16290: f64, t16294: f64, t16317: f64, t16325: f64, t16331: f64, t16338: f64, t16341: f64, t19868: f64, t19873: f64, t19876: f64, t19879: f64, t19882: f64, t19886: f64, t3778: f64, t3803: f64, t5246: f64, t5252: f64, t6390: f64, t6417: f64) -> (f64, f64, f64) {
    let t19889 = t16311 * t16225;
    let t19890 = t16305 * t19889;
    let t19893 = t1825 * t5308;
    let t19894 = t16224 * t19893;
    let t19899 = t12286 * t6390 / 1536.0_f64 - t3778 * t6417 / 3072.0_f64 - t1341 * t19868 / 3072.0_f64 - t16239 + t16241 + t5246 * t19873 / 512.0_f64 + t19876 * t5252 / 768.0_f64 - t16269 + t16290 - 7.0_f64 / 576.0_f64 * t19879 + t3803 * t19882 / 768.0_f64 - t16294 + t3803 * t19886 / 384.0_f64 - t5246 * t19890 / 192.0_f64 - 5.0_f64 / 384.0_f64 * t3803 * t19894 - 119.0_f64 / 1728.0_f64 * t16317 + t16325 + t16331 + t16338 - 35.0_f64 / 108.0_f64 * t16341;
    (t19890, t19894, t19899)
}
