//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3860/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3860(t543: f64, t74077: f64, t74165: f64, t221: f64, t22253: f64, t4018: f64, t4019: f64, t1388: f64, t1390: f64, t1410: f64, t3829: f64, t6816: f64, t74010: f64, t74015: f64, t74017: f64, t74022: f64, t74024: f64, t74029: f64, t74033: f64, t74037: f64, t828: f64, t9942: f64) -> (f64, f64) {
    let t74167 = (t74077 + t74165) * t543;
    let t74174 = t4018 * t4019 * t221 * t22253;
    let t74176 = -0.10164000561857065645e-3_f64 * t74010 - 0.28582678745379824648e-3_f64 * t74015 + 0.15244095330869239812e-3_f64 * t74017 + 0.14291339372689912324e-4_f64 * t74022 + 0.60976381323476959249e-3_f64 * t74024 - 0.57165357490759649296e-4_f64 * t74029 + 0.28582678745379824648e-4_f64 * t74033 + 0.14291339372689912324e-4_f64 * t74037 - 0.25724410870841842183e-1_f64 * t1410 * t9942 * t828 * t6816 * t3829 - 0.21437009059034868486e-3_f64 * t1388 * t1390 * t828 * t74167 - 0.25410001404642664112e-4_f64 * t74174;
    (t74167, t74176)
}
