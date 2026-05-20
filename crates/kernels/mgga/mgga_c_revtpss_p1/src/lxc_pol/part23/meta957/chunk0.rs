//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3201/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3201<F: Float>(t12772: F, t24797: F, t3625: F, t21004: F, t21030: F, t21121: F, t57707: F, t57710: F, t59233: F, t59411: F, t71738: F, t71740: F, t71742: F, t71744: F, t71749: F, t71751: F) -> F {
    let t84061 = t3625 * t12772 * t24797;
    let t84066 = F::cast_from(0.95275595817932748825e-4_f64) * t59233 - F::cast_from(0.42874018118069736972e-3_f64) * t71738 - F::cast_from(0.85748036236139473944e-3_f64) * t71740 - F::cast_from(0.85748036236139473944e-3_f64) * t71742 + F::cast_from(0.13719685797782315831e-1_f64) * t57707 * t21121 - F::cast_from(0.68598428988911579154e-2_f64) * t57710 * t21030 + F::cast_from(0.25724410870841842184e-2_f64) * t59411 * t21004 - F::cast_from(0.57165357490759649296e-3_f64) * t84061 - F::cast_from(0.48272968547752592737e-2_f64) * t71744 + F::cast_from(0.14481890564325777821e-1_f64) * t71749 - F::cast_from(0.45732285992607719436e-2_f64) * t71751;
    t84066
}
