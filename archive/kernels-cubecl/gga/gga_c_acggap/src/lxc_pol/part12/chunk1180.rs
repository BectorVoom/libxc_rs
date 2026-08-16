//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1180/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1180<F: Float>(t34794: F, t34804: F, t30730: F, t30738: F, t30744: F, t30748: F, t30750: F, t30756: F, t30758: F, t30763: F, t30767: F, t34798: F, t34802: F, t34817: F, t34821: F, t34826: F, t34830: F) -> F {
    let t37249 = F::cast_from(0.31448092289604152068e-2_f64) * t34794;
    let t37252 = F::cast_from(0.20965394859736101378e-2_f64) * t34804;
    let t37266 = t37249 + F::cast_from(0.15724046144802076034e-2_f64) * t34798 + F::cast_from(0.20965394859736101378e-2_f64) * t34802 + t37252 - F::cast_from(0.12579236915841660828e-2_f64) * t30730 - F::cast_from(0.62896184579208304137e-2_f64) * t30738 - F::cast_from(0.18868855373762491241e-2_f64) * t30744 + F::cast_from(0.83861579438944405516e-3_f64) * t30748 - F::cast_from(0.34299214494455789578e-2_f64) * t30750 + F::cast_from(0.34299214494455789578e-2_f64) * t30756 + F::cast_from(0.12579236915841660828e-2_f64) * t30758 + F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t30763 + F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t30767 - F::cast_from(0.62896184579208304138e-2_f64) * t34817 - F::cast_from(0.37737710747524982482e-2_f64) * t34821 - F::cast_from(0.25158473831683321656e-2_f64) * t34826 + F::cast_from(0.37737710747524982482e-2_f64) * t34830;
    t37266
}
