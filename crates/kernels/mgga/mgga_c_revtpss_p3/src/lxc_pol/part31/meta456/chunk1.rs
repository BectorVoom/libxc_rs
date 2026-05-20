//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1650/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1650<F: Float>(t1260: F, t6601: F, t1222: F, t1266: F, t12784: F, t12855: F, t17437: F, t21121: F, t21126: F, t21129: F, t21134: F, t21137: F, t21140: F, t5304: F, t5309: F, t5313: F, t5373: F, t5391: F, t6640: F) -> F {
    let t21143 = t6601 * t1260;
    let t21146 = -F::cast_from(0.2540682555144873302e-2_f64) * t5391 * t5304 - F::cast_from(0.28582678745379824648e-3_f64) * t12784 * t6640 - F::cast_from(0.85748036236139473944e-3_f64) * t12855 * t21121 - t17437 - F::new(2.0) / F::new(81.0) * t5373 * t5313 + t1222 * t21126 / F::new(216.0) - F::new(7.0) / F::new(648.0) * t1222 * t21129 + t5373 * t5309 / F::new(27.0) - t1222 * t21134 / F::new(144.0) - t1222 * t21137 / F::new(72.0) - t1222 * t21140 / F::new(48.0) - F::cast_from(0.14291339372689912324e-3_f64) * t21143 * t1266;
    t21146
}
