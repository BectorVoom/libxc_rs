//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1115/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1115<F: Float>(t1501: F, t3074: F, t3068: F, t1098: F, t12279: F, t12290: F, t12294: F, t12295: F, t12298: F, t12301: F, t12304: F, t12307: F, t12310: F, t12319: F, t12321: F, t3067: F, t3103: F, t3107: F, t4265: F, t9526: F, t9530: F, t9535: F, t9538: F, t9543: F, t9547: F) -> F {
    let t12324 = t1501 * t3074;
    let t12325 = t3068 * t12324;
    let t12328 = -F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1098 * t12279 + F::cast_from(5.0_f64) / F::cast_from(20736.0_f64) * t9526 - t9530 / F::cast_from(4608.0_f64) + t9535 + t9538 / F::cast_from(4608.0_f64) - t9543 / F::cast_from(6912.0_f64) + t9547 / F::cast_from(2304.0_f64) + t12290 - t12294 + t1098 * t12295 / F::cast_from(108.0_f64) + t1098 * t12298 / F::cast_from(216.0_f64) + t1098 * t12301 / F::cast_from(36.0_f64) - t1098 * t12304 / F::cast_from(72.0_f64) - t1098 * t12307 / F::cast_from(144.0_f64) - t1098 * t12310 / F::cast_from(48.0_f64) + t4265 * t3107 / F::cast_from(864.0_f64) + t4265 * t3103 / F::cast_from(432.0_f64) - t12319 - t3067 * t12321 / F::cast_from(2304.0_f64) - t3067 * t12325 / F::cast_from(4608.0_f64);
    t12328
}
