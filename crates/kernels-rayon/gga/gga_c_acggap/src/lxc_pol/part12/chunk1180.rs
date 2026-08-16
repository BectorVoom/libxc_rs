//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1180/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1180(t34794: f64, t34804: f64, t30730: f64, t30738: f64, t30744: f64, t30748: f64, t30750: f64, t30756: f64, t30758: f64, t30763: f64, t30767: f64, t34798: f64, t34802: f64, t34817: f64, t34821: f64, t34826: f64, t34830: f64) -> f64 {
    let t37249 = 0.31448092289604152068e-2_f64 * t34794;
    let t37252 = 0.20965394859736101378e-2_f64 * t34804;
    let t37266 = t37249 + 0.15724046144802076034e-2_f64 * t34798 + 0.20965394859736101378e-2_f64 * t34802 + t37252 - 0.12579236915841660828e-2_f64 * t30730 - 0.62896184579208304137e-2_f64 * t30738 - 0.18868855373762491241e-2_f64 * t30744 + 0.83861579438944405516e-3_f64 * t30748 - 0.34299214494455789578e-2_f64 * t30750 + 0.34299214494455789578e-2_f64 * t30756 + 0.12579236915841660828e-2_f64 * t30758 + 7.0_f64 / 36.0_f64 * t30763 + 7.0_f64 / 72.0_f64 * t30767 - 0.62896184579208304138e-2_f64 * t34817 - 0.37737710747524982482e-2_f64 * t34821 - 0.25158473831683321656e-2_f64 * t34826 + 0.37737710747524982482e-2_f64 * t34830;
    t37266
}
