//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1674/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1674<F: Float>(t22856: F, t22861: F, t24058: F, t24060: F, t24061: F, t26306: F, t26310: F, t26312: F, t26314: F, t26320: F, t26324: F, t22767: F, t22780: F, t22799: F, t22805: F, t24049: F, t24050: F, t26234: F, t26236: F, t26238: F, t26240: F, t26246: F, t26249: F, t26286: F, t26290: F, t26293: F, t26295: F, t26299: F, t26303: F, t27012: F, t27019: F, t27032: F) -> F {
    let t27049 = t26306 / F::cast_from(192.0_f64) + t26310 / F::cast_from(384.0_f64) - t26312 / F::cast_from(768.0_f64) + t26314 / F::cast_from(192.0_f64) + F::cast_from(0.67287926823567318088e-4_f64) * t22856 + t24058 - t22861 + t24060 + t24061 + F::cast_from(0.80745512188280781706e-3_f64) * t26320 - F::cast_from(0.40372756094140390853e-3_f64) * t26324;
    let t27051 = t27012 - t26234 / F::cast_from(768.0_f64) - t26236 / F::cast_from(768.0_f64) - t26238 / F::cast_from(768.0_f64) + F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t26240 + t22767 + F::cast_from(0.67287926823567318088e-4_f64) * t26246 + t26249 / F::cast_from(768.0_f64) - t27019 + F::cast_from(0.28260929265898273597e-2_f64) * t22780 + t27032 + t22799 + F::cast_from(0.16956557559538964158e-1_f64) * t22805 - t24049 + t24050 + t26286 / F::cast_from(8.0_f64) + F::cast_from(0.16956557559538964158e-1_f64) * t26290 - F::cast_from(0.40372756094140390853e-3_f64) * t26293 + F::cast_from(0.28260929265898273597e-2_f64) * t26295 + F::cast_from(0.24223653656484234512e-2_f64) * t26299 + F::cast_from(0.24223653656484234512e-2_f64) * t26303 + t27049;
    t27051
}
