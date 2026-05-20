//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3860/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3860<F: Float>(t543: F, t74077: F, t74165: F, t221: F, t22253: F, t4018: F, t4019: F, t1388: F, t1390: F, t1410: F, t3829: F, t6816: F, t74010: F, t74015: F, t74017: F, t74022: F, t74024: F, t74029: F, t74033: F, t74037: F, t828: F, t9942: F) -> (F, F) {
    let t74167 = (t74077 + t74165) * t543;
    let t74174 = t4018 * t4019 * t221 * t22253;
    let t74176 = -F::cast_from(0.10164000561857065645e-3_f64) * t74010 - F::cast_from(0.28582678745379824648e-3_f64) * t74015 + F::cast_from(0.15244095330869239812e-3_f64) * t74017 + F::cast_from(0.14291339372689912324e-4_f64) * t74022 + F::cast_from(0.60976381323476959249e-3_f64) * t74024 - F::cast_from(0.57165357490759649296e-4_f64) * t74029 + F::cast_from(0.28582678745379824648e-4_f64) * t74033 + F::cast_from(0.14291339372689912324e-4_f64) * t74037 - F::cast_from(0.25724410870841842183e-1_f64) * t1410 * t9942 * t828 * t6816 * t3829 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t1390 * t828 * t74167 - F::cast_from(0.25410001404642664112e-4_f64) * t74174;
    (t74167, t74176)
}
