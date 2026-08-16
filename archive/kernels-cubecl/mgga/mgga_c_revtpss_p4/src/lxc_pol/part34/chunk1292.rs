//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1292/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1292<F: Float>(t100272: F, t100329: F, t100343: F, t100365: F, t107101: F, t107107: F, t107140: F, t107154: F, t107169: F, t107188: F, t1972: F, t23499: F, t23869: F, t23874: F, t23878: F, t23913: F, t23917: F, t23960: F, t23966: F, t23980: F, t24017: F, t25517: F, t27526: F, t27531: F, t27536: F, t375: F, t7111: F, t7132: F) -> F {
    let t113667 = -t107101 / F::cast_from(144.0_f64) + F::cast_from(0.42874018118069736972e-3_f64) * t23960 * t1972 * t375 + F::cast_from(0.85748036236139473944e-3_f64) * t107107 - F::cast_from(0.28582678745379824648e-3_f64) * t100272 + t27526 * t27531 * t23499 / F::cast_from(72.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t107140 + t107154 / F::cast_from(288.0_f64) + F::cast_from(0.1270341277572436651e-2_f64) * t7132 * t23980 + F::cast_from(0.25724410870841842183e-2_f64) * t27536 * t23966 + F::cast_from(0.28582678745379824648e-3_f64) * t100329 - F::cast_from(0.19055119163586549765e-3_f64) * t100343 + t7111 * t24017 / F::cast_from(48.0_f64) + t7111 * t23869 / F::cast_from(288.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t7111 * t23874 + t107169 / F::cast_from(216.0_f64) - t7111 * t23878 / F::cast_from(36.0_f64) + F::cast_from(0.85748036236139473944e-3_f64) * t25517 * t23913 + F::cast_from(0.14291339372689912324e-2_f64) * t25517 * t23917 - F::cast_from(0.17149607247227894789e-2_f64) * t107188 - t100365 / F::cast_from(432.0_f64);
    t113667
}
