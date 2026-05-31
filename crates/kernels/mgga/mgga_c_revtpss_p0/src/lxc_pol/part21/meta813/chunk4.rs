//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2981/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2981<F: Float>(t15994: F, t3241: F, t43537: F, t53668: F, t11933: F, t16035: F, t11774: F, t127: F, t15585: F, t4872: F, t11247: F, t11631: F, t11662: F, t11697: F, t11703: F, t15691: F, t15696: F, t15700: F, t16027: F, t16089: F, t3117: F, t42310: F, t42695: F, t42788: F, t43082: F, t43297: F, t4573: F, t4907: F, t53670: F, t53822: F) -> F {
    let t54314 = t3241 * t15994;
    let t54316 = t43537 * t53668;
    let t54324 = t11933 * t16035;
    let t54341 = t11774 * t127 * t4872 * t15585;
    let t54346 = -F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t54314 - F::cast_from(0.77173232612525526552e-2_f64) * t54316 * t3117 * t53670 * t11631 * t11247 - F::cast_from(0.21722835846488666732e-1_f64) * t42695 * t4907 + F::cast_from(0.45732285992607719436e-2_f64) * t54324 - F::cast_from(0.68598428988911579154e-2_f64) * t43297 * t16027 - F::cast_from(0.42874018118069736972e-3_f64) * t11774 * t15696 * t11697 - F::cast_from(0.85748036236139473944e-3_f64) * t43082 * t15696 * t11662 + F::cast_from(0.14291339372689912324e-2_f64) * t16089 * t11703 * t4573 * t53822 + F::cast_from(0.30488190661738479624e-2_f64) * t42788 - F::cast_from(0.57165357490759649295e-3_f64) * t54341 - F::cast_from(0.85748036236139473944e-3_f64) * t15700 * t15691 * t42310;
    t54346
}
