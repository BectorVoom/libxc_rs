//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2222/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2222<F: Float>(t15775: F, t7132: F, t100054: F, t3299: F, t100030: F, t15158: F, t15586: F, t15611: F, t15697: F, t16027: F, t16123: F, t16223: F, t16230: F, t1659: F, t25553: F, t27526: F, t27527: F, t375: F, t7111: F, t93658: F, t93667: F, t93752: F, t93799: F, t93801: F) -> F {
    let t100289 = F::cast_from(0.6351706387862183255e-3_f64) * t7132 * t15775;
    let t100302 = t3299 * t100054;
    let t100310 = F::cast_from(0.14481890564325777821e-1_f64) * t1659 * t25553 * t375 + t100289 - F::cast_from(0.57165357490759649296e-3_f64) * t93752 * t15697 - F::cast_from(0.57165357490759649296e-3_f64) * t93752 * t15586 + F::cast_from(0.95275595817932748826e-3_f64) * t100030 * t16223 - F::cast_from(0.30488190661738479624e-2_f64) * t93799 - F::cast_from(0.19055119163586549765e-3_f64) * t93801 + F::cast_from(0.85748036236139473944e-3_f64) * t93667 * t16027 + t7111 * t16123 / F::cast_from(288.0_f64) + F::cast_from(0.11433071498151929859e-2_f64) * t100302 * t16230 + t27526 * t27527 * t15158 / F::cast_from(48.0_f64) - F::cast_from(0.17149607247227894789e-2_f64) * t93658 * t15611;
    t100310
}
