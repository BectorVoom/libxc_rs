//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3201/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3201(t12772: f64, t24797: f64, t3625: f64, t21004: f64, t21030: f64, t21121: f64, t57707: f64, t57710: f64, t59233: f64, t59411: f64, t71738: f64, t71740: f64, t71742: f64, t71744: f64, t71749: f64, t71751: f64) -> f64 {
    let t84061 = t3625 * t12772 * t24797;
    let t84066 = 0.95275595817932748825e-4_f64 * t59233 - 0.42874018118069736972e-3_f64 * t71738 - 0.85748036236139473944e-3_f64 * t71740 - 0.85748036236139473944e-3_f64 * t71742 + 0.13719685797782315831e-1_f64 * t57707 * t21121 - 0.68598428988911579154e-2_f64 * t57710 * t21030 + 0.25724410870841842184e-2_f64 * t59411 * t21004 - 0.57165357490759649296e-3_f64 * t84061 - 0.48272968547752592737e-2_f64 * t71744 + 0.14481890564325777821e-1_f64 * t71749 - 0.45732285992607719436e-2_f64 * t71751;
    t84066
}
