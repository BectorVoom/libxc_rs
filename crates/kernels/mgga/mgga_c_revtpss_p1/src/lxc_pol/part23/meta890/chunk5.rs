//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2836/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2836<F: Float>(t1559: F, t18608: F, t2661: F, t2662: F, t23253: F, t40348: F, t10777: F, t10779: F, t5984: F, t10786: F, t18426: F, t18435: F, t18627: F, t18632: F, t2745: F, t2747: F, t40673: F, t40722: F, t4362: F, t4364: F, t4424: F, t4450: F, t50774: F, t50957: F, t61701: F, t61888: F, t61890: F, t61892: F, t61913: F, t61916: F, t61924: F, t61952: F, t61959: F, t76284: F) -> F {
    let t76645 = t2661 * t2662 * t18608 * t1559;
    let t76647 = t40348 * t23253;
    let t76672 = t10777 * t10779 * t5984 * t1559;
    let t76676 = F::cast_from(0.27107389498472794075e-3_f64) * t61888 - F::cast_from(0.68026775414003982661e-1_f64) * t61890 - F::cast_from(0.22866142996303859719e-3_f64) * t61892 - F::cast_from(0.1829520101134271816e-3_f64) * t40722 - F::cast_from(0.15246000842785598468e-2_f64) * t61913 + F::cast_from(0.30492001685571196935e-3_f64) * t61916 - F::cast_from(0.85748036236139473942e-4_f64) * t76645 + F::cast_from(0.60023625365297631763e-2_f64) * t76647 - F::cast_from(0.91464571985215438872e-3_f64) * t61924 - F::cast_from(0.77173232612525526552e-1_f64) * t50957 * t40673 * t4450 * t18435 - F::cast_from(0.51448821741683684368e-2_f64) * t4362 * t2747 * t76284 * t10786 + F::cast_from(0.38586616306262763276e-2_f64) * t4362 * t4364 * t18426 * t18632 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t61701 * t1559 + F::cast_from(0.25724410870841842183e-2_f64) * t2745 * t2747 * t18627 * t4424 + F::cast_from(0.15246000842785598467e-3_f64) * t76672 - F::cast_from(0.60023625365297631762e-2_f64) * t61952 + F::cast_from(0.15246000842785598467e-3_f64) * t61959 + t50774;
    t76676
}
