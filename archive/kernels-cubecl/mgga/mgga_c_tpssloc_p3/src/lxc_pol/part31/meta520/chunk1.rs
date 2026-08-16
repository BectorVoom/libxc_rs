//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1729/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1729<F: Float>(t20085: F, t2095: F, t24432: F, t28830: F, t23957: F, t28826: F, t26231: F, t26246: F, t26251: F, t26255: F, t26266: F, t26268: F, t28058: F, t28061: F, t28063: F, t28065: F, t28068: F, t28070: F, t28074: F, t28078: F, t28080: F) -> (F, F, F, F) {
    let t29243 = t2095 * t20085;
    let t29247 = t24432 * t28830;
    let t29252 = t23957 * t28826;
    let t29274 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t26231 + F::cast_from(0.13457585364713463618e-3_f64) * t26246 - F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t26251 + F::cast_from(0.80745512188280781706e-3_f64) * t28058 - F::cast_from(0.40372756094140390853e-3_f64) * t28061 - t28063 / F::cast_from(768.0_f64) - t28065 / F::cast_from(384.0_f64) - F::cast_from(0.40372756094140390853e-3_f64) * t28068 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t26255 + t28070 / F::cast_from(8.0_f64) + F::cast_from(0.16956557559538964158e-1_f64) * t28074 - F::cast_from(0.24223653656484234512e-2_f64) * t28078 - t28080 / F::cast_from(24.0_f64) + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t26266 + F::cast_from(0.33913115119077928316e-1_f64) * t26268;
    (t29243, t29247, t29252, t29274)
}
