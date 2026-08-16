//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 353/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk353<F: Float>(t1668: F, t373: F, t1045: F, t1042: F, t1066: F, t1592: F, t247: F, t1009: F, t1011: F, t1025: F, t1041: F, t1060: F, t1063: F, t1656: F, t1660: F, t1665: F, t375: F) -> (F, F, F, F) {
    let t1669 = t373 * t1668;
    let t1670 = t1669 * t1045;
    let t1671 = t1042 * t1670;
    let t1674 = t1066 * t1592;
    let t1675 = t247 * t1674;
    let t1678 = t1009 + t1011 * t1656 / F::cast_from(288.0_f64) + F::cast_from(0.21437009059034868486e-3_f64) * t1660 * t375 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t1665 + F::cast_from(0.21437009059034868486e-3_f64) * t1041 * t1671 + t1060 + F::cast_from(0.14291339372689912324e-3_f64) * t1063 * t1675;
    (t1670, t1671, t1675, t1678)
}
