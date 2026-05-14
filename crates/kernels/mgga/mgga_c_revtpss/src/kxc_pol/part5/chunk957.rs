//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 957/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk957<F: Float>(t221: F, t346: F, t68: F, t345: F, t245: F, t3089: F, t3088: F, t3114: F, t11223: F, t225: F, t366: F, t1026: F, t371: F, t676: F, t1025: F, t271: F, t2857: F) -> (F, F, F, F, F, F, F, F) {
    let t11735 = t221 * t68 * t346;
    let t11737 = 5.0 / 1296.0 * t345 * t11735;
    let t11772 = t3089 * t245;
    let t11773 = t3088 * t11772;
    let t11774 = t3114 * t11773;
    let t11788 = t11223 * t225;
    let t11789 = t11788 * t366;
    let t11817 = t371 * t676 * t1026;
    let t11818 = t1025 * t11817;
    let t11821 = 1.0 / t271 / t2857;
    (t11737, t11772, t11773, t11774, t11788, t11789, t11818, t11821)
}
