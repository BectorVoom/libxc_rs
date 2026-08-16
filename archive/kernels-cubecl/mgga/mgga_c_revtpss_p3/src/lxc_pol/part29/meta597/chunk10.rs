//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2025/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2025<F: Float>(t2722: F, t7997: F, t103452: F, t103462: F, t103463: F, t103467: F, t103471: F, t14978: F, t2061: F, t2067: F, t231: F, t25317: F, t25391: F, t27312: F, t27353: F, t28309: F, t51570: F, t7070: F, t7071: F, t7076: F, t886: F, t95825: F, t95888: F, t95891: F, t95893: F, t95899: F, t95900: F, t99300: F) -> (F, F) {
    let t103483 = t7997 * t2722;
    let t103488 = F::cast_from(0.26020884564615598386e1_f64) * t27353 * t103452 * t51570 + F::cast_from(0.34270468708064099208e-1_f64) * t95888 + t95891 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t95825 * t27312 - t103462 + F::cast_from(0.17135234354032049604e-2_f64) * t103463 - t95893 + t95899 + t103467 - F::cast_from(0.25702851531048074406e-1_f64) * t95900 + F::cast_from(0.4818682326780666368e-3_f64) * t103471 - F::cast_from(0.4336814094102599731e0_f64) * t99300 * t2067 - F::cast_from(0.52041769129231196772e1_f64) * t7070 * t25317 * t28309 * t886 + F::cast_from(0.8673628188205199462e0_f64) * t7070 * t7071 * t2061 * t14978 + F::cast_from(0.4336814094102599731e0_f64) * t7070 * t7076 * t103483 * t231;
    (t103483, t103488)
}
