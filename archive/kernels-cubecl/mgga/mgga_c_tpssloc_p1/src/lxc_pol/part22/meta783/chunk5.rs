//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2684/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2684<F: Float>(t74699: F, t74735: F, t74754: F, t74765: F, t225: F, t1307: F, t16305: F, t16311: F, t19876: F, t19890: F, t19966: F, t40124: F, t40145: F, t5246: F, t54534: F, t554: F, t559: F, t57127: F, t57143: F, t57145: F, t57158: F, t57160: F, t57170: F, t6414: F, t74677: F) -> (F, F, F) {
    let t74767 = t74699 + t74735 + t74754 + t74765;
    let t74768 = t74767 * t225;
    let t74786 = -t5246 * t16305 * t16311 * t6414 * t1307 / F::cast_from(128.0_f64) - t19876 * t19890 / F::cast_from(64.0_f64) + t74768 * t554 * t559 / F::cast_from(3072.0_f64) + F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t40124 - F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t40145 - t54534 + F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t57127 + t19876 * t19966 / F::cast_from(512.0_f64) + F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t57143 - F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t57145 + F::cast_from(7.0_f64) / F::cast_from(4.0_f64) * t57158 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t57160 - F::cast_from(7.0_f64) / F::cast_from(16.0_f64) * t57170 - t5246 * t16305 * t16311 * t74677 / F::cast_from(64.0_f64);
    (t74767, t74768, t74786)
}
