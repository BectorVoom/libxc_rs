//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2113/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2113<F: Float>(t18482: F, t25270: F, t18478: F, t27261: F, t18531: F, t25245: F, t18432: F, t93025: F, t18440: F, t25227: F, t2661: F, t103287: F, t106030: F, t106033: F, t106035: F, t106037: F, t106040: F, t106042: F, t99012: F) -> F {
    let t106044 = t25270 * t18482;
    let t106046 = t27261 * t18478;
    let t106048 = t25245 * t18531;
    let t106050 = t93025 * t18432;
    let t106053 = t2661 * t25227 * t18440;
    let t106055 = t99012 - F::cast_from(0.28582678745379824648e-4_f64) * t106030 + F::cast_from(0.14291339372689912324e-4_f64) * t106033 + t103287 - F::cast_from(0.17149607247227894789e-2_f64) * t106035 - F::cast_from(0.10164000561857065645e-3_f64) * t106037 + F::cast_from(0.14291339372689912324e-4_f64) * t106040 + F::cast_from(0.20007875121765877254e-2_f64) * t106042 - F::cast_from(0.17149607247227894789e-1_f64) * t106044 - F::cast_from(0.68598428988911579156e-2_f64) * t106046 - F::cast_from(0.25410001404642664113e-4_f64) * t106048 + F::cast_from(0.50820002809285328225e-4_f64) * t106050 - F::cast_from(0.11433071498151929859e-3_f64) * t106053;
    t106055
}
