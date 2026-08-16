//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1657/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1657<F: Float>(t1260: F, t6601: F, t1222: F, t1266: F, t12784: F, t12855: F, t17437: F, t21121: F, t21126: F, t21129: F, t21134: F, t21137: F, t21140: F, t5304: F, t5309: F, t5313: F, t5373: F, t5391: F, t6640: F) -> F {
    let t21143 = t6601 * t1260;
    let t21146 = -F::cast_from(0.2540682555144873302e-2_f64) * t5391 * t5304 - F::cast_from(0.28582678745379824648e-3_f64) * t12784 * t6640 - F::cast_from(0.85748036236139473944e-3_f64) * t12855 * t21121 - t17437 - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t5373 * t5313 + t1222 * t21126 / F::cast_from(216.0_f64) - F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1222 * t21129 + t5373 * t5309 / F::cast_from(27.0_f64) - t1222 * t21134 / F::cast_from(144.0_f64) - t1222 * t21137 / F::cast_from(72.0_f64) - t1222 * t21140 / F::cast_from(48.0_f64) - F::cast_from(0.14291339372689912324e-3_f64) * t21143 * t1266;
    t21146
}
