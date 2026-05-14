//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 882/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk882<F: Float>(t11725: F, t247: F, t2853: F, t1063: F, t1007: F, t3083: F, t1003: F, t3080: F, t221: F, t346: F, t68: F, t345: F, t10345: F, t344: F, t2858: F, t3109: F) -> (F, F, F, F, F, F) {
    let t11727 = t247 * t11725 * t2853;
    let t11728 = t1063 * t11727;
    let t11730 = t3083 * t1007;
    let t11732 = t1003 * t3080;
    let t11735 = t221 * t68 * t346;
    let t11737 = 5.0 / 1296.0 * t345 * t11735;
    let t11738 = t10345 * t344;
    let t11744 = t247 * t3109 * t2858;
    (t11728, t11730, t11732, t11737, t11738, t11744)
}
