//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1803/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1803<F: Float>(t16225: F, t16311: F, t16305: F, t1825: F, t5308: F, t16224: F, t12286: F, t1341: F, t16239: F, t16241: F, t16269: F, t16290: F, t16294: F, t16317: F, t16325: F, t16331: F, t16338: F, t16341: F, t19868: F, t19873: F, t19876: F, t19879: F, t19882: F, t19886: F, t3778: F, t3803: F, t5246: F, t5252: F, t6390: F, t6417: F) -> (F, F, F) {
    let t19889 = t16311 * t16225;
    let t19890 = t16305 * t19889;
    let t19893 = t1825 * t5308;
    let t19894 = t16224 * t19893;
    let t19899 = t12286 * t6390 / F::cast_from(1536.0_f64) - t3778 * t6417 / F::cast_from(3072.0_f64) - t1341 * t19868 / F::cast_from(3072.0_f64) - t16239 + t16241 + t5246 * t19873 / F::cast_from(512.0_f64) + t19876 * t5252 / F::cast_from(768.0_f64) - t16269 + t16290 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t19879 + t3803 * t19882 / F::cast_from(768.0_f64) - t16294 + t3803 * t19886 / F::cast_from(384.0_f64) - t5246 * t19890 / F::cast_from(192.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t3803 * t19894 - F::cast_from(119.0_f64) / F::cast_from(1728.0_f64) * t16317 + t16325 + t16331 + t16338 - F::cast_from(35.0_f64) / F::cast_from(108.0_f64) * t16341;
    (t19890, t19894, t19899)
}
