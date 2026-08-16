//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2025/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2025(t2722: f64, t7997: f64, t103452: f64, t103462: f64, t103463: f64, t103467: f64, t103471: f64, t14978: f64, t2061: f64, t2067: f64, t231: f64, t25317: f64, t25391: f64, t27312: f64, t27353: f64, t28309: f64, t51570: f64, t7070: f64, t7071: f64, t7076: f64, t886: f64, t95825: f64, t95888: f64, t95891: f64, t95893: f64, t95899: f64, t95900: f64, t99300: f64) -> (f64, f64) {
    let t103483 = t7997 * t2722;
    let t103488 = 0.26020884564615598386e1_f64 * t27353 * t103452 * t51570 + 0.34270468708064099208e-1_f64 * t95888 + t95891 - 0.17347256376410398924e1_f64 * t25391 * t95825 * t27312 - t103462 + 0.17135234354032049604e-2_f64 * t103463 - t95893 + t95899 + t103467 - 0.25702851531048074406e-1_f64 * t95900 + 0.4818682326780666368e-3_f64 * t103471 - 0.4336814094102599731e0_f64 * t99300 * t2067 - 0.52041769129231196772e1_f64 * t7070 * t25317 * t28309 * t886 + 0.8673628188205199462e0_f64 * t7070 * t7071 * t2061 * t14978 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t103483 * t231;
    (t103483, t103488)
}
