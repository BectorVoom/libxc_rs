//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1249/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1249<F: Float>(t30: F, t33: F, t13584: F, t9375: F, t6785: F, t9335: F, t3833: F, t5824: F, t18280: F, t2255: F, t513: F, t5549: F, t605: F, t6792: F, t9350: F, t3841: F, t6416: F, t1113: F, t20256: F, t516: F, t5557: F, zeta_threshold: F) -> (F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t21901 = 40.0 * t13584;
    let t21905 = 0.5848223622634646207e0 * t9375;
    let t21906 = t9335 * t6785;
    let t21911 = t3833 * t5824;
    let t21917 = piecewise3(t31, 0.0, -8.0 / 27.0 * t21906 * t605 + 16.0 / 9.0 * t5549 * t2255 + 4.0 / 9.0 * t21911 * t605 + 4.0 / 3.0 * t513 * t18280);
    let t21918 = t9350 * t6792;
    let t21923 = t3841 * t6416;
    let t21929 = piecewise3(t34, 0.0, -8.0 / 27.0 * t21918 * t1113 - 16.0 / 9.0 * t5557 * t2255 + 4.0 / 9.0 * t21923 * t1113 + 4.0 / 3.0 * t516 * t20256);
    (t21901, t21905, t21917, t21929)
}
