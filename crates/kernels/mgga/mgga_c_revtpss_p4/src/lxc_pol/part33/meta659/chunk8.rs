//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2136/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2136<F: Float>(t105945: F, t7063: F, t7060: F, t29637: F, t786: F, t789: F, t27199: F, t27317: F, t27322: F, t7775: F, t93306: F, t93324: F, t99303: F, t99391: F, t99406: F, t99412: F, t99420: F, t99423: F, t99425: F, t99435: F) -> F {
    let t106387 = t7063 * t105945;
    let t106388 = t106387 * t7060;
    let t106395 = t786 * t29637 * t789;
    let t106403 = -t99391 - t99406 + F::cast_from(0.38549458614245330944e-1_f64) * t99412 + F::cast_from(0.17135234354032049604e-1_f64) * t93306 - F::cast_from(0.12851425765524037203e-1_f64) * t106388 - t99420 + F::cast_from(0.96373646535613327359e-3_f64) * t99423 - F::cast_from(0.45699670022203476294e-2_f64) * t99425 + F::cast_from(0.8673628188205199462e0_f64) * t99303 * t7775 + F::cast_from(0.9757440539382783019e-2_f64) * t106395 + F::cast_from(0.23131639038696784278e-2_f64) * t99435 + F::cast_from(0.17135234354032049604e-1_f64) * t93324 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t27322 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t27317;
    t106403
}
