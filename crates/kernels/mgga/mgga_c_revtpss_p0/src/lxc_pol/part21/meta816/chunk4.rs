//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2999/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2999<F: Float>(t15731: F, t3169: F, t11281: F, t11774: F, t11883: F, t1469: F, t15707: F, t15725: F, t15804: F, t16149: F, t3241: F, t372: F, t42926: F, t42929: F, t42932: F, t42947: F, t42962: F, t4801: F, t4916: F, t54398: F) -> F {
    let t54733 = t3169 * t15731;
    let t54735 = -F::new(11.0) / F::new(54.0) * t11883 * t4916 - t3241 * t15804 / F::new(6.0) - F::cast_from(0.28582678745379824648e-3_f64) * t42926 - F::cast_from(0.28582678745379824648e-3_f64) * t42929 + F::cast_from(0.14291339372689912324e-3_f64) * t42932 + F::cast_from(0.85748036236139473944e-3_f64) * t15725 * t16149 + F::cast_from(0.17149607247227894789e-2_f64) * t11774 * t372 * t4801 * t1469 * t54398 - F::cast_from(0.42874018118069736972e-3_f64) * t42947 - F::cast_from(0.42874018118069736972e-3_f64) * t15707 * t11281 + F::cast_from(0.57165357490759649295e-3_f64) * t42962 + F::cast_from(0.7622047665434619906e-3_f64) * t54733;
    t54735
}
