//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1400/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1400<F: Float>(t6071: F, t72: F, t686: F, t2465: F, t213: F, t6041: F, t6048: F, t10995: F, t10987: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11030: F, t15018: F, t15047: F, t15050: F, t887: F) -> (F, F, F) {
    let t18796 = t6071 * t72;
    let t18797 = t18796 * t686;
    let t18798 = t2465 * t18797;
    let t18800 = t213 * t6041;
    let t18804 = t6048 * t72;
    let t18805 = t18804 * t686;
    let t18806 = t10995 * t18805;
    let t18810 = F::cast_from(0.23131639038696784278e-2_f64) * t15018 - t10987 - F::cast_from(0.73171657588172351096e-2_f64) * t11000 + F::cast_from(0.65049603595885220126e-3_f64) * t11004 - F::cast_from(0.9757440539382783019e-2_f64) * t18798 - F::cast_from(0.65854491829355115987e0_f64) * t18800 * t887 - F::cast_from(0.13009920719177044025e-1_f64) * t11013 + t11017 + F::cast_from(0.19514881078765566037e-1_f64) * t18806 + F::cast_from(0.11565819519348392139e-2_f64) * t11019 + t15047 + t15050 - F::cast_from(0.65049603595885220126e-3_f64) * t11030;
    (t18797, t18805, t18810)
}
