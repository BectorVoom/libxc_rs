//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1489/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1489<F: Float>(t1041: F, t11622: F, t3172: F, t12021: F, t3173: F, t1032: F, t1040: F, t11902: F, t11762: F, t3241: F, t1047: F, t11659: F, t11703: F, t11705: F, t11714: F, t11883: F, t3177: F, t3238: F, t3248: F, t3255: F, t42216: F, t42227: F, t4892: F, t4899: F) -> F {
    let t42230 = t1041 * t3172 * t11622;
    let t42232 = t12021 * t3173;
    let t42235 = t11902 * t1032 * t1040;
    let t42240 = t3241 * t11762;
    let t42246 = F::cast_from(0.28582678745379824648e-2_f64) * t4892 * t11703 * t11659 * t42216 - F::cast_from(0.14291339372689912324e-2_f64) * t4899 * t11703 * t11659 * t11705 - F::cast_from(0.91464571985215438872e-2_f64) * t11714 * t3177 + F::cast_from(0.17149607247227894789e-2_f64) * t42227 + F::cast_from(0.57165357490759649296e-3_f64) * t42230 + F::cast_from(0.17149607247227894789e-2_f64) * t42232 + F::cast_from(0.85748036236139473944e-3_f64) * t42235 * t1047 - F::cast_from(11.0_f64) / F::cast_from(27.0_f64) * t11883 * t3238 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t42240 + F::cast_from(11.0_f64) / F::cast_from(54.0_f64) * t11883 * t3248 + F::cast_from(22.0_f64) / F::cast_from(81.0_f64) * t11883 * t3255;
    t42246
}
